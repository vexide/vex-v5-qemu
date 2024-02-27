use super::types::*;

pub unsafe extern "C" fn optical_hue(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn optical_sat(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn optical_brightness(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn optical_proximity(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn optical_rgb(device: V5DeviceHandle, rgb: *mut OpticalRgb) {
    unsafe {
        *rgb = OpticalRgb {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            brightness: 0.0,
        };
    }
}
pub unsafe extern "C" fn set_optical_led_pwm(device: V5DeviceHandle, value: i32) {}
pub unsafe extern "C" fn optical_led_pwm(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn optical_status(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn optical_raw(device: V5DeviceHandle, raw_data: *mut OpticalRaw) {
    unsafe {
        *raw_data = OpticalRaw {
            clear: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
// pub unsafe extern "C" fn vexDeviceOpticalDebugGet(device: V5DeviceHandle) {}
pub unsafe extern "C" fn set_optical_mode(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn optical_mode(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn optical_gesture(
    device: V5DeviceHandle,
    gesture: *mut OpticalGesture,
) -> u32 {
    0
}
pub unsafe extern "C" fn optical_gesture_enable(device: V5DeviceHandle) {}
pub unsafe extern "C" fn optical_gesture_disable(device: V5DeviceHandle) {}
pub unsafe extern "C" fn optical_proximity_threshold(
    device: V5DeviceHandle,
    threshold: i32,
) -> i32 {
    0
}
// pub unsafe extern "C" fn vexDeviceOpticalGainSet(device: V5DeviceHandle, gain: f64) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixSet(device: V5DeviceHandle, matrix: *mut OpticalMatrix) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixGet(device: V5DeviceHandle, matrix: *mut OpticalMatrix) {}
pub unsafe extern "C" fn set_optical_integration_time(device: V5DeviceHandle, millis: f64) {}
pub unsafe extern "C" fn optical_integration_time(device: V5DeviceHandle) -> f64 {
    0.0
}
