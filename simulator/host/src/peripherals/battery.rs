use vex_v5_qemu_protocol::{battery::BatteryData, KernelBoundPacket};

use crate::connection::{ConnectionError, QemuConnection};

#[derive(Default, Debug)]
pub struct Battery {
    connection: QemuConnection,
    data: BatteryData,
}

impl Battery {
    pub(crate) fn new(connection: QemuConnection, data: BatteryData) -> Self {
        Self { connection, data }
    }

    async fn update(&mut self) -> Result<(), ConnectionError> {
        self.connection
            .send_packet(KernelBoundPacket::BatteryUpdate {
                data: self.data,
                timestamp: 0,
            })
            .await
    }

    pub async fn set_voltage(&mut self, voltage: i32) -> Result<(), ConnectionError> {
        self.data.voltage = voltage;
        self.update().await
    }

    pub async fn set_capacity(&mut self, capacity: f64) -> Result<(), ConnectionError> {
        self.data.capacity = capacity;
        self.update().await
    }

    pub async fn set_current(&mut self, current: i32) -> Result<(), ConnectionError> {
        self.data.current = current;
        self.update().await
    }

    pub async fn set_temperature(&mut self, temperature: f64) -> Result<(), ConnectionError> {
        self.data.temperature = temperature;
        self.update().await
    }

    pub fn voltage(&self) -> i32 {
        self.data.voltage
    }

    pub fn capacity(&self) -> f64 {
        self.data.capacity
    }

    pub fn current(&self) -> i32 {
        self.data.current
    }

    pub fn temperature(&self) -> f64 {
        self.data.temperature
    }
}
