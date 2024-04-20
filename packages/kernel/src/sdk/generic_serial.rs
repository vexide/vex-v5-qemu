//! Smart Port Generic Serial Communication

use vex_sdk::*;

pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32) {}
pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32) {}
pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialTransmit(device: V5_DeviceT, buffer: *const u8, length: i32) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialReceive(device: V5_DeviceT, buffer: *mut u8, length: i32) -> i32 {
    Default::default()
}
pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT) {}
