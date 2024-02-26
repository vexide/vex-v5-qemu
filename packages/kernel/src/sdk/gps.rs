use super::types::*;

pub unsafe extern "C" fn vexDeviceGpsReset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceGpsHeadingGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsDegreesGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsQuaternionGet(device: V5DeviceHandle, quat: *mut GpsQuaternion) {}
pub unsafe extern "C" fn vexDeviceGpsAttitudeGet(
    device: V5DeviceHandle,
    attitude: *mut GpsAttitude,
    is_raw: bool,
) {
}
pub unsafe extern "C" fn vexDeviceGpsRawGyroGet(device: V5DeviceHandle, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn vexDeviceGpsRawAccelGet(device: V5DeviceHandle, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn vexDeviceGpsStatusGet(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceGpsTemperatureGet(device: V5DeviceHandle) -> f64 {}
// pub unsafe extern "C" fn vexDeviceGpsDebugGet(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceGpsModeSet(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn vexDeviceGpsModeGet(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceGpsDataRateSet(device: V5DeviceHandle, rate: u32) {}
pub unsafe extern "C" fn vexDeviceGpsOriginSet(device: V5DeviceHandle, x: f64, y: f64) {}
pub unsafe extern "C" fn vexDeviceGpsOriginGet(device: V5DeviceHandle, x: *mut f64, y: *mut f64) {}
pub unsafe extern "C" fn vexDeviceGpsRotationSet(device: V5DeviceHandle, rotation: f64) {}
pub unsafe extern "C" fn vexDeviceGpsRotationGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsInitialPositionSet(
    device: V5DeviceHandle,
    initial_x: f64,
    initial_y: f64,
    initial_rotation: f64,
) {
}
pub unsafe extern "C" fn vexDeviceGpsErrorGet(device: V5DeviceHandle) -> f64 {
    0.0
}
