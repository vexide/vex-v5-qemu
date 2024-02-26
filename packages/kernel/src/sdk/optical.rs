use super::types::*;

pub unsafe extern "C" fn vexDeviceOpticalHueGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalSatGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalBrightnessGet(device: V5Device) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalProximityGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalRgbGet(device: V5Device, rgb: *mut OpticalRgb) {
    unsafe {
        *rgb = OpticalRgb {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            brightness: 0.0,
        };
    }
}
pub unsafe extern "C" fn vexDeviceOpticalLedPwmSet(device: V5Device, value: i32) {}
pub unsafe extern "C" fn vexDeviceOpticalLedPwmGet(device: V5Device) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalStatusGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalRawGet(device: V5Device, raw_data: *mut OpticalRaw) {
    unsafe {
        *raw_data = OpticalRaw {
            clear: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
// pub unsafe extern "C" fn vexDeviceOpticalDebugGet(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceOpticalModeSet(device: V5Device, mode: u32) {}
pub unsafe extern "C" fn vexDeviceOpticalModeGet(device: V5Device) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalGestureGet(
    device: V5Device,
    gesture: *mut OpticalGesture,
) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalGestureEnable(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceOpticalGestureDisable(device: V5Device) {}
pub unsafe extern "C" fn vexDeviceOpticalProximityThreshold(
    device: V5Device,
    threshold: i32,
) -> i32 {
    0
}
// pub unsafe extern "C" fn vexDeviceOpticalGainSet(device: V5Device, gain: f64) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixSet(device: V5Device, matrix: *mut OpticalMatrix) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixGet(device: V5Device, matrix: *mut OpticalMatrix) {}
pub unsafe extern "C" fn vexDeviceOpticalIntegrationTimeSet(device: V5Device, millis: f64) {}
pub unsafe extern "C" fn vexDeviceOpticalIntegrationTimeGet(device: V5Device) -> f64 {
    0.0
}
