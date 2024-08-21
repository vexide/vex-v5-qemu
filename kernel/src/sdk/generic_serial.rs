//! Smart Port Generic Serial Communication

use vex_sdk::*;

pub extern "C" fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32) {}
pub extern "C" fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32) {}
pub extern "C" fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialTransmit(
    device: V5_DeviceT,
    buffer: *const u8,
    length: i32,
) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialReceive(
    device: V5_DeviceT,
    buffer: *mut u8,
    length: i32,
) -> i32 {
    Default::default()
}
pub extern "C" fn vexDeviceGenericSerialFlush(device: V5_DeviceT) {}
