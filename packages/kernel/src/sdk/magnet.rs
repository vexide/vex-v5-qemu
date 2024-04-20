//! V5 Workcell Electromagnet

use core::ffi::c_double;

use vex_sdk::*;

pub fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32) {}
pub fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration) {}
pub fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration) {}
pub fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
