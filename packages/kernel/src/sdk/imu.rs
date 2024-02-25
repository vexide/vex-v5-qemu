use super::types::*;

pub unsafe extern "C" fn vexDeviceImuReset(device: V5Device) {

}
pub unsafe extern "C" fn vexDeviceImuHeadingGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceImuDegreesGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceImuQuaternionGet(device: V5Device, data: *mut ImuQuaternion) { 

}
pub unsafe extern "C" fn vexDeviceImuAttitudeGet(device: V5Device, data: *mut ImuAttitude) {

}
pub unsafe extern "C" fn vexDeviceImuRawGyroGet(device: V5Device, data: *mut ImuRaw) {

}
pub unsafe extern "C" fn vexDeviceImuRawAccelGet(device: V5Device, data: *mut ImuRaw) {

}
pub unsafe extern "C" fn vexDeviceImuStatusGet(device: V5Device) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuTemperatureGet(device: V5Device) -> f64 {
//     0.0
// }
// pub unsafe extern "C" fn vexDeviceImuDebugGet(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceImuModeSet(device: V5Device, mode: u32 ) {

}
pub unsafe extern "C" fn vexDeviceImuModeGet(device: V5Device) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceImuCollisionDataGet(device: V5Device) -> ImuCollisionData {}
pub unsafe extern "C" fn vexDeviceImuDataRateSet(device: V5Device, rate: u32) {

}