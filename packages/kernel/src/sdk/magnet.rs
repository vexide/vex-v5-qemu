use super::types::*;

pub unsafe extern "C" fn vexDeviceMagnetPowerSet(device: V5DeviceHandle, value: i32, time: i32) {}
pub unsafe extern "C" fn vexDeviceMagnetPowerGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMagnetPickup(device: V5DeviceHandle, duration: MagnetDuration) {}
pub unsafe extern "C" fn vexDeviceMagnetDrop(device: V5DeviceHandle, duration: MagnetDuration) {}
pub unsafe extern "C" fn vexDeviceMagnetTemperatureGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMagnetCurrentGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMagnetStatusGet(device: V5DeviceHandle) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceMagnetDebugGet(device: V5DeviceHandle) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeSet(device: V5DeviceHandle, mode: MagnetMode) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeGet(device: V5DeviceHandle) -> MagnetMode {
