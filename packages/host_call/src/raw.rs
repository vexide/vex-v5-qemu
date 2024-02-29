//! Raw types stored in guest-physical memory.
//!
//! These types must be valid on both the host and guest,
//! so they must be `repr(C)` and use only raw pointers.
//!
//! This makes them unsuitable for direct use by the user,
//! so they are wrapped in safe types in the parent module.

use core::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicU8};

#[repr(C)]
#[derive(Debug)]
pub struct CallTable {
    pub call_cells: [CallCell; crate::CALL_CELLS],
}

#[repr(C)]
#[derive(Debug)]
pub struct CallCell {
    pub status: AtomicU8,
    pub content: UnsafeCell<MaybeUninit<Call>>,
}

impl CallCell {
    pub const STATUS_READY: u8 = 0;
    pub const STATUS_PENDING: u8 = 1;
}

#[repr(C)]
#[derive(Debug)]
pub enum Call {
    Write {
        data: *const [u8],
        written: *mut u32,
    },
}
