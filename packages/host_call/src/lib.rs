#![no_std]
#![feature(const_mut_refs)]

pub mod perspective;
mod raw;

use core::{marker::PhantomData, mem::MaybeUninit, ptr, sync::atomic::Ordering};

use perspective::Perspective;

/// Represents the guest's memory.
pub struct Guest<'g, P: Perspective> {
    memory: *mut (),
    call_cells: Option<&'g [raw::CallCell; 64]>,
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
    pub const unsafe fn new_on_host(memory: *mut u8) -> Self {
        Self::new(memory)
    }

    /// Convert a guest-relative pointer to a host-relative pointer.
    ///
    /// # Safety
    /// - `T` may not be more aligned than the guest memory.
    unsafe fn guest_relative_ptr<T: ?Sized>(&self, ptr: *const T) -> *const T {
        unsafe { ptr.byte_add(self.memory as usize) }
    }

    /// Convert a guest-relative mutable pointer to a host-relative mutable pointer.
    ///
    /// # Safety
    /// - `T` may not be more aligned than the guest memory.
    unsafe fn guest_relative_mut<T: ?Sized>(&self, ptr: *mut T) -> *mut T {
        unsafe { ptr.byte_add(self.memory as usize) }
    }
}

impl<'g, P: Perspective> Guest<'g, P> {
    /// Create a new [`Guest`] from the given guest memory offset.
    const unsafe fn new(guest_memory: *mut u8) -> Self {
        let call_table = &*guest_memory.add(0x03700000).cast::<raw::CallTable>();

        Self {
            memory: guest_memory.cast(),
            call_cells: Some(&call_table.call_cells),
            _perspective: PhantomData,
        }
    }

    /// Take the call cells.
    pub fn take_call_cells(&mut self) -> Option<[ReadyCallCell<'_, P>; CALL_CELLS]> {
        self.call_cells.take().map(|raw_cells| {
            raw_cells.each_ref().map(|raw| ReadyCallCell {
                guest: self,
                raw,
                _perspective: PhantomData,
            })
        })
    }
}

pub struct ReadyCallCell<'g, P: Perspective> {
    guest: &'g Guest<'g, P>,
    raw: &'g raw::CallCell,
    _perspective: PhantomData<P>,
}

impl<'g> ReadyCallCell<'g, perspective::Guest> {
    pub fn perform<'c>(self, mut call: Call<'c>) -> PendingCallCell<'g, 'c, perspective::Guest> {
        unsafe {
            self.raw
                .content
                .get()
                .write(MaybeUninit::new(call.as_raw()))
        }

        PendingCallCell {
            guest: self.guest,
            raw: self.raw,
            _call: PhantomData,
            _perspective: PhantomData,
        }
    }
}

impl<'g> ReadyCallCell<'g, perspective::Host> {
    pub fn poll_incoming(self) -> Result<IncomingCall<'g>, Self> {
        match self.raw.status.load(Ordering::Acquire) {
            raw::CallCell::STATUS_READY => Ok(IncomingCall {
                cell: PendingCallCell {
                    guest: self.guest,
                    raw: self.raw,
                    _call: PhantomData,
                    _perspective: PhantomData,
                },
                call: unsafe {
                    Call::from_raw(&self.guest, (&*self.raw.content.get()).assume_init_ref())
                },
            }),
            raw::CallCell::STATUS_PENDING => Err(self),
            _ => unreachable!(),
        }
    }
}

pub struct IncomingCall<'g> {
    pub cell: PendingCallCell<'g, 'g, perspective::Host>,
    pub call: Call<'g>,
}

pub struct PendingCallCell<'g: 'c, 'c, P: Perspective> {
    guest: &'g Guest<'g, P>,
    raw: &'g raw::CallCell,
    _call: PhantomData<Call<'c>>,
    _perspective: PhantomData<P>,
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Guest> {
    pub fn poll_completion(self) -> Result<ReadyCallCell<'g, perspective::Guest>, Self> {
        match self.raw.status.load(Ordering::Acquire) {
            raw::CallCell::STATUS_READY => Ok(ReadyCallCell {
                guest: self.guest,
                raw: self.raw,
                _perspective: PhantomData,
            }),
            raw::CallCell::STATUS_PENDING => Err(self),
            _ => unreachable!(),
        }
    }
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Host> {
    pub fn complete(self) -> ReadyCallCell<'g, perspective::Host> {
        self.raw
            .status
            .store(raw::CallCell::STATUS_READY, Ordering::Release);

        ReadyCallCell {
            guest: self.guest,
            raw: self.raw,
            _perspective: PhantomData,
        }
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

pub enum Call<'c> {
    Write {
        data: &'c [u8],
        written: &'c mut u32,
    },
}

impl<'c> Call<'c> {
    unsafe fn from_raw(guest: &'c Guest<'c, perspective::Host>, raw: &'c raw::Call) -> Self {
        match *raw {
            raw::Call::Write { data, written } => Call::Write {
                data: &*guest.guest_relative_ptr(data),
                written: &mut *guest.guest_relative_mut(written),
            },
        }
    }

    fn as_raw(&mut self) -> raw::Call {
        match self {
            Call::Write { data, written } => raw::Call::Write {
                data: ptr::from_ref(data),
                written: ptr::from_mut(written),
            },
        }
    }
}

pub const CALL_CELLS: usize = 64;
