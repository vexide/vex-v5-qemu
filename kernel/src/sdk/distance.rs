//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::*;

pub extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
