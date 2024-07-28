//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use vex_sdk::*;

pub fn vexDevicesGetNumber() -> u32 {
    Default::default()
}
pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32 {
    Default::default()
}
pub fn vexDevicesGet() -> V5_DeviceT {
    core::ptr::null_mut()
}
pub fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT {
    core::ptr::null_mut()
}
pub fn vexDeviceFlagsGetByIndex(index: u32) -> u32 {
    Default::default()
}
pub fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32 {
    Default::default()
}
pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType {
    Default::default()
}
pub fn vexDeviceButtonStateGet() -> c_int {
    Default::default()
}
