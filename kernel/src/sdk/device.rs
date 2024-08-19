//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use vex_sdk::*;

pub extern "C" fn vexDevicesGetNumber() -> u32 {
    Default::default()
}
pub extern "C" fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32 {
    Default::default()
}
pub extern "C" fn vexDevicesGet() -> V5_DeviceT {
    core::ptr::null_mut()
}
pub extern "C" fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT {
    core::ptr::null_mut()
}
pub extern "C" fn vexDeviceFlagsGetByIndex(index: u32) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType {
    Default::default()
}
pub extern "C" fn vexDeviceButtonStateGet() -> c_int {
    Default::default()
}
