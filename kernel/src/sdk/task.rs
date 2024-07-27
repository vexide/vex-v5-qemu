//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

pub fn vexTaskAdd(
    callback: unsafe extern "C" fn() -> c_int,
    interval: c_int,
    label: *const c_char,
) {
}
pub fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void {
    core::ptr::null_mut()
}
pub fn vexTaskSleep(time: u32) {}
pub fn vexTaskHardwareConcurrency() -> i32 {
    Default::default()
}
pub fn vexBackgroundProcessing() {}
pub fn vexTasksRun() {}
