//! Filesystem Access

use core::ffi::c_char;

use vex_sdk::*;

pub extern "C" fn vexFileMountSD() -> FRESULT {
    Default::default()
}
pub extern "C" fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT {
    Default::default()
}
pub extern "C" fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub extern "C" fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub extern "C" fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub extern "C" fn vexFileClose(fdp: *mut FIL) {}
pub extern "C" fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    Default::default()
}
pub extern "C" fn vexFileSize(fdp: *mut FIL) -> i32 {
    Default::default()
}
pub extern "C" fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT {
    Default::default()
}
pub extern "C" fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    Default::default()
}
pub extern "C" fn vexFileDriveStatus(drive: u32) -> bool {
    Default::default()
}
pub extern "C" fn vexFileTell(fdp: *mut FIL) -> i32 {
    Default::default()
}
pub extern "C" fn vexFileSync(fdp: *mut FIL) {}
pub extern "C" fn vexFileStatus(filename: *const c_char) -> u32 {
    Default::default()
}
