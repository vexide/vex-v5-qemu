//! V5 AI Vision Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub extern "C" fn vexDeviceAiVisionClassNameGet(device: V5_DeviceT, id: i32, pName: *mut u8) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionCodeGet(
    device: V5_DeviceT,
    id: u32,
    pCode: *mut V5_DeviceAiVisionCode,
) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionCodeSet(device: V5_DeviceT, pCode: *mut V5_DeviceAiVisionCode) {}
pub extern "C" fn vexDeviceAiVisionColorGet(
    device: V5_DeviceT,
    id: u32,
    pColor: *mut V5_DeviceAiVisionColor,
) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionColorSet(device: V5_DeviceT, pColor: *mut V5_DeviceAiVisionColor) {}
pub extern "C" fn vexDeviceAiVisionModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionModeSet(device: V5_DeviceT, mode: u32) {}
pub extern "C" fn vexDeviceAiVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionObjectGet(
    device: V5_DeviceT,
    indexObj: u32,
    pObject: *mut V5_DeviceAiVisionObject,
) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionSensorSet(device: V5_DeviceT, brightness: c_double, contrast: c_double) {}
pub extern "C" fn vexDeviceAiVisionStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceAiVisionTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
