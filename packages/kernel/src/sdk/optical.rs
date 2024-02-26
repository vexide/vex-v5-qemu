use super::types::*;

pub unsafe extern "C" fn vexDeviceOpticalHueGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalSatGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalBrightnessGet(device: V5DeviceHandle) -> f64 {
    0.0
}
pub unsafe extern "C" fn vexDeviceOpticalProximityGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalRgbGet(device: V5DeviceHandle, rgb: *mut OpticalRgb) {
    unsafe {
        *rgb = OpticalRgb {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            brightness: 0.0,
        };
    }
}
pub unsafe extern "C" fn vexDeviceOpticalLedPwmSet(device: V5DeviceHandle, value: i32) {}
pub unsafe extern "C" fn vexDeviceOpticalLedPwmGet(device: V5DeviceHandle) -> i32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalStatusGet(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalRawGet(device: V5DeviceHandle, raw_data: *mut OpticalRaw) {
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
pub unsafe extern "C" fn vexDeviceOpticalModeSet(device: V5DeviceHandle, mode: u32) {}
pub unsafe extern "C" fn vexDeviceOpticalModeGet(device: V5DeviceHandle) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalGestureGet(
    device: V5DeviceHandle,
    gesture: *mut OpticalGesture,
) -> u32 {
    0
}
pub unsafe extern "C" fn vexDeviceOpticalGestureEnable(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceOpticalGestureDisable(device: V5DeviceHandle) {}
pub unsafe extern "C" fn vexDeviceOpticalProximityThreshold(
    device: V5DeviceHandle,
    threshold: i32,
) -> i32 {
    0
}
// pub unsafe extern "C" fn vexDeviceOpticalGainSet(device: V5DeviceHandle, gain: f64) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixSet(device: V5DeviceHandle, matrix: *mut OpticalMatrix) {}
// pub unsafe extern "C" fn vexDeviceOpticalMatrixGet(device: V5DeviceHandle, matrix: *mut OpticalMatrix) {}
pub unsafe extern "C" fn vexDeviceOpticalIntegrationTimeSet(device: V5DeviceHandle, millis: f64) {}
pub unsafe extern "C" fn vexDeviceOpticalIntegrationTimeGet(device: V5DeviceHandle) -> f64 {
    0.0
}
