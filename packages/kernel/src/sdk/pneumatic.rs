//! CTE Workcell Pneumatics Control

use vex_sdk::*;

pub extern "C" fn vexDevicePneumaticActuationStatusGet(
    device: V5_DeviceT,
    ac1: *mut u16,
    ac2: *mut u16,
    ac3: *mut u16,
    ac4: *mut u16,
) -> u32 {
    Default::default()
}
pub extern "C" fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool) {}
pub extern "C" fn vexDevicePneumaticCtrlSet(
    device: V5_DeviceT,
    pCtrl: *mut V5_DevicePneumaticCtrl,
) {
}
pub extern "C" fn vexDevicePneumaticCylinderPwmSet(
    device: V5_DeviceT,
    id: u32,
    bState: bool,
    pwm: u8,
) {
}
pub extern "C" fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool) {}
pub extern "C" fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
pub extern "C" fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8) {}
pub extern "C" fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
