//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use vex_sdk::*;
use vex_v5_qemu_protocol::DeviceData;

use crate::sync::Mutex;

use super::BATTERY;

pub static SMARTPORTS: [Mutex<SmartPort>; 21] = [
    Mutex::new(SmartPort::new(0)),
    Mutex::new(SmartPort::new(1)),
    Mutex::new(SmartPort::new(2)),
    Mutex::new(SmartPort::new(3)),
    Mutex::new(SmartPort::new(4)),
    Mutex::new(SmartPort::new(5)),
    Mutex::new(SmartPort::new(6)),
    Mutex::new(SmartPort::new(7)),
    Mutex::new(SmartPort::new(8)),
    Mutex::new(SmartPort::new(9)),
    Mutex::new(SmartPort::new(10)),
    Mutex::new(SmartPort::new(11)),
    Mutex::new(SmartPort::new(12)),
    Mutex::new(SmartPort::new(13)),
    Mutex::new(SmartPort::new(14)),
    Mutex::new(SmartPort::new(15)),
    Mutex::new(SmartPort::new(16)),
    Mutex::new(SmartPort::new(17)),
    Mutex::new(SmartPort::new(18)),
    Mutex::new(SmartPort::new(19)),
    Mutex::new(SmartPort::new(20)),
];

pub static ONBOARD_ADI: Mutex<SmartPort> = Mutex::new(SmartPort::new(21));

pub struct SmartPort {
    pub port_index: u8,
    pub timestamp: u32,
    pub data: Option<DeviceData>,
}

impl SmartPort {
    pub const fn new(port_index: u8) -> Self {
        Self {
            port_index,
            data: None,
            timestamp: 0,
        }
    }
}

pub trait Device {
    fn port_index(&self) -> u8;
    fn timestamp(&self) -> u32;
    fn device_type(&self) -> V5_DeviceType;
    fn handle(&self) -> V5_DeviceT;
}

impl Device for SmartPort {
    fn port_index(&self) -> u8 {
        self.port_index
    }

    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    fn device_type(&self) -> V5_DeviceType {
        V5_DeviceType::kDeviceTypeNoSensor
    }

    fn handle(&self) -> V5_DeviceT {
        let mut device = V5_Device::default();
        device.zero_indexed_port = self.port_index;
        device.one_indexed_port = self.port_index + 1;
        device.device_type = self.device_type();

        &mut device
    }
}

pub extern "C" fn vexDevicesGetNumber() -> u32 {
    Default::default()
}
pub extern "C" fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32 {
    let mut count = match device_type {
        // This is to account for onboard ADI or the battery without obtaining a lock,
        // since we already know they're always connected.
        //
        // TODO: determine if these actually contribute to the count on real hardware.
        // TODO: same deal with controllers
        V5_DeviceType::kDeviceTypeAdiSensor | V5_DeviceType::kDeviceTypeRes2Sensor => 1,
        _ => 0,
    };

    for port in SMARTPORTS.iter() {
        if port.lock().device_type() == device_type {
            count += 1;
        }
    }

    count
}
pub extern "C" fn vexDevicesGet() -> V5_DeviceT {
    core::ptr::null_mut()
}
pub extern "C" fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT {
    match index {
        0..=20 => SMARTPORTS[index as usize].lock().handle(),
        21 => ONBOARD_ADI.lock().handle(),
        24 => BATTERY.lock().handle(),
        _ => core::ptr::null_mut(),
    }
}
pub extern "C" fn vexDeviceFlagsGetByIndex(index: u32) -> u32 {
    Default::default()
}
pub extern "C" fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32 {
    Default::default()
}
pub unsafe extern "C" fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32 {
    let port_index = unsafe { *device }.zero_indexed_port;

    match port_index {
        0..=20 => SMARTPORTS[port_index as usize].lock().timestamp(),
        21 => ONBOARD_ADI.lock().timestamp(),
        24 => BATTERY.lock().timestamp(),
        _ => 0,
    }
}
pub extern "C" fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
pub extern "C" fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType {
    Default::default()
}
pub extern "C" fn vexDeviceButtonStateGet() -> c_int {
    Default::default()
}
