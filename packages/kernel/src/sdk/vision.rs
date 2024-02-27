use super::types::*;

pub unsafe extern "C" fn set_vision_mode(device: V5DeviceHandle, mode: VisionMode) {}
pub unsafe extern "C" fn vision_mode(device: V5DeviceHandle) -> VisionMode {
    VisionMode::Normal
}
pub unsafe extern "C" fn vision_object_count(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vision_object(
    device: V5DeviceHandle,
    index: u32,
    object: *mut VisionObject,
) -> i32 {
    0
}
pub unsafe extern "C" fn set_vision_signature(
    device: V5DeviceHandle,
    signature: *mut VisionSignature,
) {
}
pub unsafe extern "C" fn vision_signature(
    device: V5DeviceHandle,
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
pub unsafe extern "C" fn set_vision_brightness(device: V5DeviceHandle, percent: u8) {}
pub unsafe extern "C" fn vision_brightness(device: V5DeviceHandle) -> u8 {
    0
}
pub unsafe extern "C" fn set_vision_white_balance_mode(
    device: V5DeviceHandle,
    mode: VisionWhiteBalanceMode,
) {
}
pub unsafe extern "C" fn vision_white_balance_mode(
    device: V5DeviceHandle,
) -> VisionWhiteBalanceMode {
    VisionWhiteBalanceMode::Normal
}
pub unsafe extern "C" fn set_vision_white_balance(device: V5DeviceHandle, color: VisionRgb) {}
pub unsafe extern "C" fn vision_white_balance(device: V5DeviceHandle) -> VisionRgb {
    VisionRgb {
        red: 0,
        green: 0,
        blue: 0,
        brightness: 0,
    }
}
pub unsafe extern "C" fn set_vision_led_mod(device: V5DeviceHandle, mode: VisionLedMode) {}
pub unsafe extern "C" fn vision_led_mode(device: V5DeviceHandle) -> VisionLedMode {
    VisionLedMode::Auto
}
pub unsafe extern "C" fn set_vision_led_brigntness(device: V5DeviceHandle, percent: u8) {}
pub unsafe extern "C" fn vision_led_brigntness(device: V5DeviceHandle) -> u8 {
    0
}
pub unsafe extern "C" fn set_vision_led_color(device: V5DeviceHandle, color: VisionRgb) {}
pub unsafe extern "C" fn vision_led_color(device: V5DeviceHandle) -> VisionRgb {
    VisionRgb {
        red: 0,
        green: 0,
        blue: 0,
        brightness: 0,
    }
}
pub unsafe extern "C" fn set_vision_wifi_mode(device: V5DeviceHandle, mode: VisionWifiMode) {}
pub unsafe extern "C" fn vision_wifi_mode(device: V5DeviceHandle) -> VisionWifiMode {
    VisionWifiMode::Off
}
