//! V5 GPS

use core::ffi::c_double;
use vex_sdk::*;

pub extern "C" fn vexDeviceGpsReset(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceGpsQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceGpsQuaternion) {
}
pub extern "C" fn vexDeviceGpsAttitudeGet(
    device: V5_DeviceT,
    data: *mut V5_DeviceGpsAttitude,
    bRaw: bool,
) {
}
pub extern "C" fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {}
pub extern "C" fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {}
pub extern "C" fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceGpsTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32) {}
pub extern "C" fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32) {}
pub extern "C" fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double) {}
pub extern "C" fn vexDeviceGpsOriginGet(device: V5_DeviceT, ox: *mut c_double, oy: *mut c_double) {}
pub extern "C" fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double) {}
pub extern "C" fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceGpsInitialPositionSet(
    device: V5_DeviceT,
    initial_x: c_double,
    initial_y: c_double,
    initial_rotation: c_double,
) {
}
pub extern "C" fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
