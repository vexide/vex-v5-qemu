use super::types::*;

pub unsafe extern "C" fn vexDeviceAbsEncReset(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceAbsEncPositionSet(device: V5DeviceHandle, position: i32) {}
pub unsafe extern "C" fn vexDeviceAbsEncPositionGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncVelocityGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncAngleGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncReverseFlagSet(device: V5DeviceHandle, value: bool) {}
pub unsafe extern "C" fn vexDeviceAbsEncReverseFlagGet(device: V5DeviceHandle) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceAbsEncStatusGet(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceAbsEncTemperatureGet(device: V5DeviceHandle) -> i32 {}
// pub unsafe extern "C" fn vexDeviceAbsEncDebugGet(device: V5DeviceHandle) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeSet(device: V5DeviceHandle, mode: AbsEncoderMode) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeGet(device: V5DeviceHandle) -> AbsEncoderMode {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetSet(device: V5DeviceHandle, offset: i32) {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetGet(device: V5DeviceHandle) -> i32 {}
pub unsafe extern "C" fn vexDeviceAbsEncDataRateSet(device: V5DeviceHandle, rate: u32) {}
