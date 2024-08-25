//! V5 Smart Battery

use core::ffi::c_double;

use vex_sdk::{V5_Device, V5_DeviceT, V5_DeviceType};
use vex_v5_qemu_protocol::battery::BatteryData;

use crate::sync::Mutex;

use super::Device;

#[derive(Default, Debug, PartialEq)]
pub struct Battery {
    pub data: Option<BatteryData>,
    pub timestamp: u32,
}

pub static BATTERY: Mutex<Battery> = Mutex::new(Battery::new());

impl Battery {
    pub const INTERNAL_PORT_INDEX: u8 = 24;

    pub const fn new() -> Self {
        Self { data: None, timestamp: 0 }
    }
}

impl Device for Battery {
    fn port_index(&self) -> u8 {
        Self::INTERNAL_PORT_INDEX
    }

    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    fn device_type(&self) -> V5_DeviceType {
        V5_DeviceType::kDeviceTypeRes2Sensor
    }

    fn handle(&self) -> V5_DeviceT {
        let mut device = V5_Device::default();
        device.zero_indexed_port = Self::INTERNAL_PORT_INDEX;
        device.one_indexed_port = Self::INTERNAL_PORT_INDEX + 1;
        device.device_type = self.device_type();

        &mut device
    }
}

pub extern "C" fn vexBatteryVoltageGet() -> i32 {
    if let Some(battery_data) = BATTERY.lock().data {
        battery_data.voltage
    } else {
        -1
    }
}

pub extern "C" fn vexBatteryCurrentGet() -> i32 {
    if let Some(battery_data) = BATTERY.lock().data {
        battery_data.current
    } else {
        -1
    }
}

pub extern "C" fn vexBatteryTemperatureGet() -> c_double {
    if let Some(battery_data) = BATTERY.lock().data {
        battery_data.temperature
    } else {
        0.0
    }
}

pub extern "C" fn vexBatteryCapacityGet() -> c_double {
    if let Some(battery_data) = BATTERY.lock().data {
        battery_data.capacity
    } else {
        0.0
    }
}
