use super::types::*;

pub unsafe extern "C" fn vexDeviceGpsReset(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceGpsHeadingGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsDegreesGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsQuaternionGet(device: V5Device, quat: *mut GpsQuaternion) {}
pub unsafe extern "C" fn vexDeviceGpsAttitudeGet(
    device: V5Device,
    attitude: *mut GpsAttitude,
    is_raw: bool,
) {
}
pub unsafe extern "C" fn vexDeviceGpsRawGyroGet(device: V5Device, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn vexDeviceGpsRawAccelGet(device: V5Device, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn vexDeviceGpsStatusGet(device: V5Device) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceGpsTemperatureGet(device: V5Device) -> f64 {}
// pub unsafe extern "C" fn vexDeviceGpsDebugGet(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceGpsModeSet(device: V5Device, mode: u32) {}
pub unsafe extern "C" fn vexDeviceGpsModeGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceGpsDataRateSet(device: V5Device, rate: u32) {}
pub unsafe extern "C" fn vexDeviceGpsOriginSet(device: V5Device, x: f64, y: f64) {}
pub unsafe extern "C" fn vexDeviceGpsOriginGet(device: V5Device, x: *mut f64, y: *mut f64) {}
pub unsafe extern "C" fn vexDeviceGpsRotationSet(device: V5Device, rotation: f64) {}
pub unsafe extern "C" fn vexDeviceGpsRotationGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceGpsInitialPositionSet(
    device: V5Device,
    initial_x: f64,
    initial_y: f64,
    initial_rotation: f64,
) {
}
pub unsafe extern "C" fn vexDeviceGpsErrorGet(device: V5Device) -> f64 {
    0.0
}
