//! V5 Optical Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb) {}
pub fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32) {}
pub fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw) {}
pub fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32) {}
pub fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub fn vexDeviceOpticalGestureGet(device: V5_DeviceT, pData: *mut V5_DeviceOpticalGesture) -> u32 {
    Default::default()
}
pub fn vexDeviceOpticalGestureEnable(device: V5_DeviceT) {}
pub fn vexDeviceOpticalGestureDisable(device: V5_DeviceT) {}
pub fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32) {}
pub fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double) {}
pub fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
