//! USB Serial Communication

use core::ffi::{c_char, VaList};

pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    -1
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    -1
}
pub fn vexSerialReadChar(channel: u32) -> i32 {
    -1
}
pub fn vexSerialPeekChar(channel: u32) -> i32 {
    -1
}
pub fn vexSerialWriteFree(channel: u32) -> i32 {
    -1
}
pub fn vex_vprintf(format: *const c_char, args: VaList<'_, '_>) -> i32 {
    -1
}
pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList<'_, '_>) -> i32 {
    -1
}
pub fn vex_vsnprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    args: VaList<'_, '_>,
) -> i32 {
    -1
}
