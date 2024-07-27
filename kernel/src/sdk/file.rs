//! Filesystem Access

use core::ffi::c_char;

use vex_sdk::*;

pub fn vexFileMountSD() -> FRESULT {
    Default::default()
}
pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT {
    Default::default()
}
pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL {
    core::ptr::null_mut()
}
pub fn vexFileClose(fdp: *mut FIL) {}
pub fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    Default::default()
}
pub fn vexFileSize(fdp: *mut FIL) -> i32 {
    Default::default()
}
pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT {
    Default::default()
}
pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    Default::default()
}
pub fn vexFileDriveStatus(drive: u32) -> bool {
    Default::default()
}
pub fn vexFileTell(fdp: *mut FIL) -> i32 {
    Default::default()
}
pub fn vexFileSync(fdp: *mut FIL) {}
pub fn vexFileStatus(filename: *const c_char) -> u32 {
    Default::default()
}
