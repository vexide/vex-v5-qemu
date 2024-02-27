use super::types::*;

pub unsafe extern "C" fn imu_reset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn imu_heading(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn imu_degrees(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn imu_quaternion(device: V5DeviceHandle, data: *mut ImuQuaternion) {}
pub unsafe extern "C" fn imu_attitude(device: V5DeviceHandle, data: *mut ImuAttitude) {}
pub unsafe extern "C" fn imu_raw_gyro(device: V5DeviceHandle, data: *mut ImuRaw) {}
pub unsafe extern "C" fn imu_raw_accel(device: V5DeviceHandle, data: *mut ImuRaw) {}
pub unsafe extern "C" fn imu_status(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuTemperatureGet(device: V5DeviceHandle) -> f64 {
//     0.0
// }
// pub unsafe extern "C" fn vexDeviceImuDebugGet(device: V5DeviceHandle) {}
pub unsafe extern "C" fn set_imu_mode(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn imu_mode(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuCollisionDataGet(device: V5DeviceHandle) -> ImuCollisionData {}
pub unsafe extern "C" fn set_imu_data_rate(device: V5DeviceHandle, rate: u32) {}
