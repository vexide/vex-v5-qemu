use core::ffi::c_double;
use vex_sdk::*;

pub extern "C" fn vexDeviceAdiPortConfigSet(
    device: V5_DeviceT,
    port: u32,
    config: V5_AdiPortConfiguration,
) {
}
pub extern "C" fn vexDeviceAdiPortConfigGet(
    device: V5_DeviceT,
    port: u32,
) -> V5_AdiPortConfiguration {
    Default::default()
}
pub extern "C" fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32) {}
pub extern "C" fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceAdiAddrLedSet(
    device: V5_DeviceT,
    port: u32,
    pData: *mut u32,
    nOffset: u32,
    nLength: u32,
    options: u32,
) {
}
pub extern "C" fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState {
    Default::default()
}
pub extern "C" fn vexDeviceGyroReset(device: V5_DeviceT) {}
pub extern "C" fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
