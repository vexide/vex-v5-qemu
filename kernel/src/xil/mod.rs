//! Xilinx embeddedsw bindings.
//!
//! This module provides bindings to the Xilinx embeddedsw HAL library (libxil).

#![allow(
    unused_variables,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

pub mod exception;
pub mod gic;
pub mod time;
pub mod timer;
pub mod usbps;
pub mod wdt;

#[inline(always)]
pub unsafe fn Xil_In32(addr: u32) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

#[inline(always)]
pub unsafe fn Xil_Out32(addr: u32, value: u32) {
    core::ptr::write_volatile(addr as *mut u32, value)
}
