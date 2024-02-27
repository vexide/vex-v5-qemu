use super::types::*;

pub unsafe extern "C" fn set_magnet_power(device: V5DeviceHandle, value: i32, time: i32) {}
pub unsafe extern "C" fn magnet_power(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn magnet_pickup(device: V5DeviceHandle, duration: MagnetDuration) {}
pub unsafe extern "C" fn magnet_drop(device: V5DeviceHandle, duration: MagnetDuration) {}
pub unsafe extern "C" fn magnet_temperature(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn magnet_current(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn magnet_status(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceMagnetDebugGet(device: V5DeviceHandle) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeSet(device: V5DeviceHandle, mode: MagnetMode) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeGet(device: V5DeviceHandle) -> MagnetMode {
