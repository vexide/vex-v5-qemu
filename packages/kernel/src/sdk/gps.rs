use super::types::*;

pub unsafe extern "C" fn gps_reset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn gps_heading(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn gps_degrees(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn gps_quaternion(device: V5DeviceHandle, quat: *mut GpsQuaternion) {}
pub unsafe extern "C" fn gps_attitude(
    device: V5DeviceHandle,
    attitude: *mut GpsAttitude,
    is_raw: bool,
) {
}
pub unsafe extern "C" fn gps_raw_gyro(device: V5DeviceHandle, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn gps_raw_accel(device: V5DeviceHandle, raw_data: *mut GpsRaw) {}
pub unsafe extern "C" fn gps_status(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceGpsTemperatureGet(device: V5DeviceHandle) -> f64 {}
// pub unsafe extern "C" fn vexDeviceGpsDebugGet(device: V5DeviceHandle) {}
pub unsafe extern "C" fn set_gps_mode(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn gps_mode(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn set_gps_data_rate(device: V5DeviceHandle, rate: u32) {}
pub unsafe extern "C" fn set_gps_origin(device: V5DeviceHandle, x: f64, y: f64) {}
pub unsafe extern "C" fn gps_origin(device: V5DeviceHandle, x: *mut f64, y: *mut f64) {}
pub unsafe extern "C" fn set_gps_rotation(device: V5DeviceHandle, rotation: f64) {}
pub unsafe extern "C" fn gps_rotation(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn set_gps_initial_position(
    device: V5DeviceHandle,
    initial_x: f64,
    initial_y: f64,
    initial_rotation: f64,
) {
}
pub unsafe extern "C" fn gps_error(device: V5DeviceHandle) -> f64 {
    0.0
}
