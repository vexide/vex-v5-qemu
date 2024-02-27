use super::types::*;

pub unsafe extern "C" fn abs_enc_reset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn set_abs_enc_position(device: V5DeviceHandle, position: i32) {}
pub unsafe extern "C" fn abs_enc_position(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn abs_enc_velocity(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn abs_enc_angle(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn set_abs_enc_reversed(device: V5DeviceHandle, value: bool) {}
pub unsafe extern "C" fn abs_enc_reversed(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn abs_enc_status(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceAbsEncTemperatureGet(device: V5DeviceHandle) -> i32 {}
// pub unsafe extern "C" fn vexDeviceAbsEncDebugGet(device: V5DeviceHandle) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeSet(device: V5DeviceHandle, mode: AbsEncoderMode) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeGet(device: V5DeviceHandle) -> AbsEncoderMode {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetSet(device: V5DeviceHandle, offset: i32) {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetGet(device: V5DeviceHandle) -> i32 {}
pub unsafe extern "C" fn set_abs_enc_data_rate(device: V5DeviceHandle, rate: u32) {}
