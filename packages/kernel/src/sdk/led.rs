//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.

use vex_sdk::*;

pub extern "C" fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor) {}
pub extern "C" fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32) {}
pub extern "C" fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor {
    Default::default()
}
pub extern "C" fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
