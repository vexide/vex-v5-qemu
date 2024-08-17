//! V5 Vision Sensor

use vex_sdk::*;

pub extern "C" fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode) {}
pub extern "C" fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode {
    Default::default()
}
pub extern "C" fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceVisionObjectGet(
    device: V5_DeviceT,
    index: u32,
    object: *mut V5_DeviceVisionObject,
) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceVisionSignatureSet(device: V5_DeviceT, signature: *mut V5_DeviceVisionSignature) {}
pub extern "C" fn vexDeviceVisionSignatureGet(
    device: V5_DeviceT,
    id: u32,
    signature: *mut V5_DeviceVisionSignature,
) -> bool {
    Default::default()
}
pub extern "C" fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8) {}
pub extern "C" fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
pub extern "C" fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode) {}
pub extern "C" fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode {
    Default::default()
}
pub extern "C" fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
pub extern "C" fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
pub extern "C" fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode) {}
pub extern "C" fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode {
    Default::default()
}
pub extern "C" fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8) {}
pub extern "C" fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
pub extern "C" fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
pub extern "C" fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
pub extern "C" fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode) {}
pub extern "C" fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode {
    Default::default()
}
