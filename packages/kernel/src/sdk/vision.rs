use super::types::*;

pub unsafe extern "C" fn vexDeviceVisionModeSet(device: V5Device, mode: VisionMode) {}
pub unsafe extern "C" fn vexDeviceVisionModeGet(device: V5Device) -> VisionMode {
    VisionMode::Normal
}
pub unsafe extern "C" fn vexDeviceVisionObjectCountGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceVisionObjectGet(
    device: V5Device,
    index: u32,
    object: *mut VisionObject,
) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceVisionSignatureSet(
    device: V5Device,
    signature: *mut VisionSignature,
) {
}
pub unsafe extern "C" fn vexDeviceVisionSignatureGet(
    device: V5Device,
    id: u32,
    signature: *mut VisionSignature,
) -> bool {
    unsafe {
        *signature = VisionSignature {
            id: 0,
            flags: 0,
            pad: [0; 2],
            range: 0.0,
            u_min: 0,
            u_max: 0,
            u_mean: 0,
            v_min: 0,
            v_max: 0,
            v_mean: 0,
            m_rgb: 0,
            m_type: 0,
        };
    }
    false
}
pub unsafe extern "C" fn vexDeviceVisionBrightnessSet(device: V5Device, percent: u8) {}
pub unsafe extern "C" fn vexDeviceVisionBrightnessGet(device: V5Device) -> u8 {
    0
}
pub unsafe extern "C" fn vexDeviceVisionWhiteBalanceModeSet(
    device: V5Device,
    mode: VisionWhiteBalanceMode,
) {
}
pub unsafe extern "C" fn vexDeviceVisionWhiteBalanceModeGet(
    device: V5Device,
) -> VisionWhiteBalanceMode {
    VisionWhiteBalanceMode::Normal
}
pub unsafe extern "C" fn vexDeviceVisionWhiteBalanceSet(device: V5Device, color: VisionRgb) {}
pub unsafe extern "C" fn vexDeviceVisionWhiteBalanceGet(device: V5Device) -> VisionRgb {
    VisionRgb {
        red: 0,
        green: 0,
        blue: 0,
        brightness: 0,
    }
}
pub unsafe extern "C" fn vexDeviceVisionLedModeSet(device: V5Device, mode: VisionLedMode) {}
pub unsafe extern "C" fn vexDeviceVisionLedModeGet(device: V5Device) -> VisionLedMode {
    VisionLedMode::Auto
}
pub unsafe extern "C" fn vexDeviceVisionLedBrigntnessSet(device: V5Device, percent: u8) {}
pub unsafe extern "C" fn vexDeviceVisionLedBrigntnessGet(device: V5Device) -> u8 {
    0
}
pub unsafe extern "C" fn vexDeviceVisionLedColorSet(device: V5Device, color: VisionRgb) {}
pub unsafe extern "C" fn vexDeviceVisionLedColorGet(device: V5Device) -> VisionRgb {
    VisionRgb {
        red: 0,
        green: 0,
        blue: 0,
        brightness: 0,
    }
}
pub unsafe extern "C" fn vexDeviceVisionWifiModeSet(device: V5Device, mode: VisionWifiMode) {}
pub unsafe extern "C" fn vexDeviceVisionWifiModeGet(device: V5Device) -> VisionWifiMode {
    VisionWifiMode::Off
}
