//! V5 Inertial Sensor

use core::ffi::c_double;
use vex_sdk::*;

pub extern "C" fn vexDeviceImuReset(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion) {
}
pub extern "C" fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude) {}
pub extern "C" fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
pub extern "C" fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
pub extern "C" fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceImuTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32) {}
pub extern "C" fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32) {}
