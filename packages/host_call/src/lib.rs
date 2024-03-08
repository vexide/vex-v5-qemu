#![no_std]
#![feature(const_mut_refs)]
#![feature(strict_provenance)]
#![feature(ptr_metadata)]

mod abi;
pub mod perspective;

use core::{
    fmt::Debug,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
    sync::atomic::Ordering,
};

use abi::OnGuest;
use perspective::Perspective;

/// Represents the guest's memory.
#[derive(Debug)]
pub struct Guest<'g, P: Perspective> {
    memory: *mut (),
    call_cells: Option<&'g [abi::CallCell; 64]>,
    _perspective: PhantomData<P>,
}

impl Guest<'static, perspective::Guest> {
    /// Creates a new `Guest` from the perspective of the guest.
    pub const unsafe fn new_on_guest() -> Self {
        Self::new(ptr::null_mut())
    }
}

impl<'g> Guest<'g, perspective::Host> {
    /// Creates a new `Guest` from the perspective of the host.
    pub unsafe fn new_on_host(memory: *mut ()) -> Self {
        Self::new(memory)
    }

    /// Resolve a guest-relative pointer to a host-relative pointer.
    ///
    /// # Safety
    /// See [`abi::GuestPointer::from_guest_addr`].
    unsafe fn resolve<P: abi::GuestPointer>(&self, guest_ptr: abi::OnGuest<P>) -> P {
        P::from_guest_addr(self.memory, guest_ptr.addr, guest_ptr.metadata)
    }
}

impl<'g, P: Perspective> Guest<'g, P> {
    /// Create a new [`Guest`] from the given guest memory.
    const unsafe fn new(memory: *mut ()) -> Self {
        let call_table = &*memory
            .byte_add(abi::CallTable::OFFSET)
            .cast::<abi::CallTable>();

        Self {
            memory,
            call_cells: Some(&call_table.call_cells),
            _perspective: PhantomData,
        }
    }

    /// Take the call cells.
    pub fn take_call_cells(&mut self) -> Option<[VacantCallCell<'_, P>; CALL_CELLS]> {
        self.call_cells.take().map(|raw_cells| {
            raw_cells.each_ref().map(|raw| VacantCallCell {
                guest: self,
                raw,
                _perspective: PhantomData,
            })
        })
    }
}

pub struct VacantCallCell<'g, P: Perspective> {
    guest: &'g Guest<'g, P>,
    raw: &'g abi::CallCell,
    _perspective: PhantomData<P>,
}

impl<P: Perspective> Debug for VacantCallCell<'_, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VacantCallCell")
            .field("raw", self.raw)
            .finish()
    }
}

impl<'g> VacantCallCell<'g, perspective::Guest> {
    pub fn perform<'c>(self, mut call: Call<'c>) -> PendingCallCell<'g, 'c, perspective::Guest> {
        unsafe {
            self.raw
                .content
                .get()
                .write_volatile(MaybeUninit::new(call.as_raw()))
        }

        self.raw
            .status
            .store(abi::CallCell::STATUS_PENDING, Ordering::Release);

        PendingCallCell {
            guest: self.guest,
            raw: self.raw,
            _call: PhantomData,
            _perspective: PhantomData,
        }
    }
}

impl<'g> VacantCallCell<'g, perspective::Host> {
    pub fn poll_incoming(self) -> Result<IncomingCall<'g>, Self> {
        match self.raw.status.load(Ordering::Acquire) {
            abi::CallCell::STATUS_VACANT => Err(self),
            abi::CallCell::STATUS_PENDING => Ok(IncomingCall {
                cell: PendingCallCell {
                    guest: self.guest,
                    raw: self.raw,
                    _call: PhantomData,
                    _perspective: PhantomData,
                },
                call: unsafe {
                    Call::from_raw(
                        &self.guest,
                        self.raw.content.get().read_volatile().assume_init(),
                    )
                },
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct IncomingCall<'g> {
    pub cell: PendingCallCell<'g, 'g, perspective::Host>,
    pub call: Call<'g>,
}

pub struct PendingCallCell<'g: 'c, 'c, P: Perspective> {
    guest: &'g Guest<'g, P>,
    raw: &'g abi::CallCell,
    _call: PhantomData<Call<'c>>,
    _perspective: PhantomData<P>,
}

impl<P: Perspective> Debug for PendingCallCell<'_, '_, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PendingCallCell")
            .field("raw", self.raw)
            .finish()
    }
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Guest> {
    pub fn poll_completion(self) -> Result<VacantCallCell<'g, perspective::Guest>, Self> {
        match self.raw.status.load(Ordering::Acquire) {
            abi::CallCell::STATUS_VACANT => Ok(VacantCallCell {
                guest: self.guest,
                raw: self.raw,
                _perspective: PhantomData,
            }),
            abi::CallCell::STATUS_PENDING => Err(self),
            _ => unreachable!(),
        }
    }
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Host> {
    pub fn complete(self) -> VacantCallCell<'g, perspective::Host> {
        self.raw
            .status
            .store(abi::CallCell::STATUS_VACANT, Ordering::Release);

        let ret = VacantCallCell {
            guest: self.guest,
            raw: self.raw,
            _perspective: PhantomData,
        };

        mem::forget(self);

        ret
    }
}

// We can't drop pending calls, because they must remain to prevent the guest
// from mutating/deallocating data referenced by the call.
// Ideally this would only be implemented for guest-perspective pending calls,
// but it's not possible to implement `Drop` for a specialization of a type.
impl<'g, 'c, P: Perspective> Drop for PendingCallCell<'g, 'c, P> {
    fn drop(&mut self) {
        panic!("pending calls must not be dropped")
    }
}

#[derive(Debug)]
pub enum Call<'c> {
    Write {
        data: &'c [u8],
        written: &'c mut u32,
    },
}

impl<'c> Call<'c> {
    unsafe fn from_raw(guest: &'c Guest<'c, perspective::Host>, raw: abi::Call) -> Self {
        match raw {
            abi::Call::Write { data, written } => Call::Write {
                data: &*guest.resolve(data),
                written: &mut *guest.resolve(written),
            },
        }
    }

    fn as_raw(&mut self) -> abi::Call {
        match self {
            Call::Write { data, written } => abi::Call::Write {
                data: OnGuest::new(*data as _),
                written: OnGuest::new(*written as _),
            },
        }
    }
}

pub const CALL_CELLS: usize = 64;
