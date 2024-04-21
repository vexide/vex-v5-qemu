//! USB Serial Communication

use core::ffi::{c_char, VaList};

pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    Default::default()
}
pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    Default::default()
}
pub fn vexSerialReadChar(channel: u32) -> i32 {
    Default::default()
}
pub fn vexSerialPeekChar(channel: u32) -> i32 {
    Default::default()
}
pub fn vexSerialWriteFree(channel: u32) -> i32 {
    Default::default()
}
pub fn vex_vprintf(format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
pub fn vex_vsnprintf(out: *mut c_char, max_len: u32, format: *const c_char, args: VaList) -> i32 {
    Default::default()
}
