//! USB Serial Communication

use alloc::vec;
use core::{
    ffi::{c_char, VaList},
    slice,
};

use vex_v5_qemu_protocol::HostBoundPacket;

use crate::protocol;

pub extern "C" fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    if protocol::send_packet(HostBoundPacket::UserSerial(vec![c])).is_err() {
        -1
    } else {
        1
    }
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    if data_len == 0 {
        return 0;
    }

    if protocol::send_packet(HostBoundPacket::UserSerial(
        unsafe { slice::from_raw_parts(data, data_len as usize) }.to_vec(),
    ))
    .is_err()
    {
        -1
    } else {
        data_len as i32
    }
}
pub extern "C" fn vexSerialReadChar(channel: u32) -> i32 {
    -1
}
pub extern "C" fn vexSerialPeekChar(channel: u32) -> i32 {
    -1
}
pub extern "C" fn vexSerialWriteFree(channel: u32) -> i32 {
    -1
}
pub extern "C" fn vex_vprintf(format: *const c_char, args: VaList<'_, '_>) -> i32 {
    -1
}
pub extern "C" fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList<'_, '_>) -> i32 {
    -1
}
pub extern "C" fn vex_vsnprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    args: VaList<'_, '_>,
) -> i32 {
    -1
}
