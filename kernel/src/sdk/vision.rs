//! V5 Vision Sensor

use vex_sdk::*;

pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode) {}
pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode {
    Default::default()
}
pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceVisionObjectGet(
    device: V5_DeviceT,
    index: u32,
    object: *mut V5_DeviceVisionObject,
) -> i32 {
    Default::default()
}
pub fn vexDeviceVisionSignatureSet(device: V5_DeviceT, signature: *mut V5_DeviceVisionSignature) {}
pub fn vexDeviceVisionSignatureGet(
    device: V5_DeviceT,
    id: u32,
    signature: *mut V5_DeviceVisionSignature,
) -> bool {
    Default::default()
}
pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8) {}
pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode) {}
pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode {
    Default::default()
}
pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode) {}
pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode {
    Default::default()
}
pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8) {}
pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode) {}
pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode {
    Default::default()
}
