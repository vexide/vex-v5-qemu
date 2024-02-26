use super::types::*;

pub unsafe extern "C" fn vexDeviceMagnetPowerSet(device: V5Device, value: i32, time: i32) {}
pub unsafe extern "C" fn vexDeviceMagnetPowerGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceMagnetPickup(device: V5Device, duration: MagnetDuration) {}
pub unsafe extern "C" fn vexDeviceMagnetDrop(device: V5Device, duration: MagnetDuration) {}
pub unsafe extern "C" fn vexDeviceMagnetTemperatureGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMagnetCurrentGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceMagnetStatusGet(device: V5Device) -> u32 {
    0
}
// pub unsafe extern "C" fn vexDeviceMagnetDebugGet(device: V5Device) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeSet(device: V5Device, mode: MagnetMode) {}
// pub unsafe extern "C" fn vexDeviceMagnetModeGet(device: V5Device) -> MagnetMode {
