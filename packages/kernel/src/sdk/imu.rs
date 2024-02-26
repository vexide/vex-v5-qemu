use super::types::*;

pub unsafe extern "C" fn vexDeviceImuReset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceImuHeadingGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceImuDegreesGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceImuQuaternionGet(device: V5DeviceHandle, data: *mut ImuQuaternion) {}
pub unsafe extern "C" fn vexDeviceImuAttitudeGet(device: V5DeviceHandle, data: *mut ImuAttitude) {}
pub unsafe extern "C" fn vexDeviceImuRawGyroGet(device: V5DeviceHandle, data: *mut ImuRaw) {}
pub unsafe extern "C" fn vexDeviceImuRawAccelGet(device: V5DeviceHandle, data: *mut ImuRaw) {}
pub unsafe extern "C" fn vexDeviceImuStatusGet(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuTemperatureGet(device: V5DeviceHandle) -> f64 {
//     0.0
// }
// pub unsafe extern "C" fn vexDeviceImuDebugGet(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceImuModeSet(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn vexDeviceImuModeGet(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuCollisionDataGet(device: V5DeviceHandle) -> ImuCollisionData {}
pub unsafe extern "C" fn vexDeviceImuDataRateSet(device: V5DeviceHandle, rate: u32) {}
