//! Raw types stored in guest-physical memory.
//!
//! These types must be valid on both the host and guest,
//! so they must be `repr(C)` and use only raw pointers.
//!
//! This makes them unsuitable for direct use by the user,
//! so they are wrapped in safe types in the parent module.

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{self, Pointee},
    sync::atomic::AtomicU8,
};

#[repr(C)]
#[derive(Debug)]
pub struct CallTable {
    pub call_cells: [CallCell; crate::CALL_CELLS],
}

impl CallTable {
    pub const OFFSET: usize = 0x3700000;
}

#[repr(C)]
#[derive(Debug)]
pub struct CallCell {
    pub status: AtomicU8,
    pub content: UnsafeCell<MaybeUninit<Call>>,
}

impl CallCell {
    pub const STATUS_VACANT: u8 = 0;
    pub const STATUS_PENDING: u8 = 1;
}

#[repr(C)]
pub enum Call {
    Write {
        data: OnGuest<*const [u8]>,
        written: OnGuest<*mut u32>,
    },
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OnGuest<P: GuestPointer> {
    pub addr: u32,
    pub metadata: <P::Target as Pointee>::Metadata,
    pub _pointer: PhantomData<P>,
}

impl<P: GuestPointer> OnGuest<P> {
    /// Encode a guest pointer for use in the ABI.
    pub fn new(host_ptr: P) -> Self {
        let (addr, metadata) = host_ptr.to_raw().to_raw_parts();
        Self {
            addr: addr as u32,
            metadata,
            _pointer: PhantomData,
        }
    }
}

/// A pointer which can be converted to and created from a guest-space memory address.
pub trait GuestPointer {
    type Target: ?Sized;

    fn to_raw(self) -> *mut Self::Target;

    /// Create an instance of the pointer from a guest-relative address and metadata.
    ///
    /// # Safety
    /// - The address must be the offset of a guest memory location containing a valid instance of `Self::Target`.
    /// - The metadata must be valid for the given address.
    /// - `guest_provenance` must be a valid pointer to the start of the guest memory, and must have provenance over the entire guest memory.
    unsafe fn from_guest_addr(
        guest_provenance: *mut (),
        addr: u32,
        metadata: <Self::Target as Pointee>::Metadata,
    ) -> Self;
}

impl<T: ?Sized> GuestPointer for *const T {
    type Target = T;

    fn to_raw(self) -> *mut T {
        self.cast_mut()
    }

    unsafe fn from_guest_addr(
        guest_provenance: *mut (),
        addr: u32,
        metadata: <Self::Target as Pointee>::Metadata,
    ) -> Self {
        ptr::from_raw_parts(guest_provenance.byte_add(addr as usize), metadata)
    }
}

impl<T> GuestPointer for *mut T {
    type Target = T;

    fn to_raw(self) -> Self {
        self
    }

    unsafe fn from_guest_addr(
        guest_provenance: *mut (),
        addr: u32,
        metadata: <Self::Target as Pointee>::Metadata,
    ) -> Self {
        ptr::from_raw_parts_mut(guest_provenance.byte_add(addr as usize), metadata)
    }
}
