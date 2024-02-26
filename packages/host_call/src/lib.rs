#![no_std]
#![feature(const_mut_refs)]

pub mod perspective;
mod raw;

use core::{marker::PhantomData, ptr, sync::atomic::Ordering};

use perspective::Perspective;

/// Represents the guest's memory.
pub struct Guest<'g, P: Perspective> {
    call_table: &'g raw::CallTable,
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
}

impl<'g, P: Perspective> Guest<'g, P> {
    /// Create a new [`Guest`] from the given guest memory offset.
    const unsafe fn new(guest_memory: *mut u8) -> Self {
        let call_table = &*guest_memory.add(0x03700000).cast::<raw::CallTable>();

        Self {
            call_table,
            call_cells: Some(&call_table.call_cells),
            _perspective: PhantomData,
        }
    }

    /// Take the call cells.
    pub fn take_call_cells(&mut self) -> Option<[ReadyCallCell<'_, P>; CALL_CELLS]> {
        self.call_cells.take().map(|raw_cells| {
            raw_cells.each_ref().map(|raw| ReadyCallCell {
                raw,
                _perspective: PhantomData,
            })
        })
    }
}

pub struct ReadyCallCell<'g, P: Perspective> {
    raw: &'g raw::CallCell,
    _perspective: PhantomData<P>,
}

impl<'g> ReadyCallCell<'g, perspective::Guest> {
    pub fn perform<'c>(self, call: Call<'c>) -> PendingCallCell<'g, 'c, perspective::Guest> {
        todo!()
    }
}

impl<'g> ReadyCallCell<'g, perspective::Host> {
    pub fn poll_incoming(self) -> Result<IncomingCall<'g>, Self> {
        todo!()
    }
}

pub struct IncomingCall<'g> {
    cell: PendingCallCell<'g, 'g, perspective::Host>,
    call: Call<'g>,
}

pub struct PendingCallCell<'g: 'c, 'c, P: Perspective> {
    raw: &'g raw::CallCell,
    _call: PhantomData<Call<'c>>,
    _perspective: PhantomData<P>,
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Guest> {
    pub fn poll_completion(self) -> Result<ReadyCallCell<'g, perspective::Guest>, Self> {
        todo!()
    }
}

impl<'g, 'c> PendingCallCell<'g, 'c, perspective::Host> {
    pub fn complete(self) -> ReadyCallCell<'g, perspective::Host> {
        todo!()
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

pub const CALL_CELLS: usize = 64;
