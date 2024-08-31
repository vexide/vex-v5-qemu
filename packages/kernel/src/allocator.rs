//! VexOS heap allocator implemented with the `talc` crate.
//! [`init_heap`] must be called before any heap allocations are made.
//! This is done automatically in the `vex-startup` crate,
//! so you should not need to call it yourself unless you are writing your own
//! startup implementation.

use core::ptr::addr_of_mut;

use talc::{ErrOnOom, Span, Talc, Talck};

use crate::sync::mutex::RawMutex;

extern "C" {
    static mut __heap_start: u8;
    static mut __heap_end: u8;
}

#[global_allocator]
static ALLOCATOR: Talck<RawMutex, ErrOnOom> = Talc::new(ErrOnOom).lock();

/// Initializes the heap allocator.
///
/// # Safety
///
/// This function can only be called once.
pub unsafe fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .claim(Span::new(
                addr_of_mut!(__heap_start),
                addr_of_mut!(__heap_end),
            ))
            .unwrap();
    }
}
