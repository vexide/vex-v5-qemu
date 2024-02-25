use super::types::*;

pub unsafe extern "C" fn vexDeviceAbsEncReset(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceAbsEncPositionSet(device: V5Device, position: i32) {}
pub unsafe extern "C" fn vexDeviceAbsEncPositionGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncVelocityGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncAngleGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceAbsEncReverseFlagSet(device: V5Device, value: bool) {}
pub unsafe extern "C" fn vexDeviceAbsEncReverseFlagGet(device: V5Device) -> bool {
    false
}
pub unsafe extern "C" fn vexDeviceAbsEncStatusGet(device: V5Device) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceAbsEncTemperatureGet(device: V5Device) -> i32 {}
// pub unsafe extern "C" fn vexDeviceAbsEncDebugGet(device: V5Device) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeSet(device: V5Device, mode: AbsEncoderMode) {}
// pub unsafe extern "C" fn vexDeviceAbsEncModeGet(device: V5Device) -> AbsEncoderMode {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetSet(device: V5Device, offset: i32) {}
// pub unsafe extern "C" fn vexDeviceAbsEncOffsetGet(device: V5Device) -> i32 {}
pub unsafe extern "C" fn vexDeviceAbsEncDataRateSet(device: V5Device, rate: u32) {}
