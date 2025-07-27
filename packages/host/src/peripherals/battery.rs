use tokio::sync::mpsc::Sender;
use vex_v5_qemu_protocol::{battery::BatteryData, KernelBoundPacket};

pub struct Battery {
    data: BatteryData,
    tx: Sender<KernelBoundPacket>,
}

impl Battery {
    pub(crate) fn new(
        tx: Sender<KernelBoundPacket>,
    ) -> Self {
        Self {
            data: BatteryData::default(),
            tx,
        }
    }

    async fn update(&mut self) {
        self.tx.send(KernelBoundPacket::BatteryUpdate {
            data: self.data,
            timestamp: 0,
        }).await.unwrap(); // OK to unwrap, since the channel can't be closed.
    }

    pub async fn set_data(&mut self, data: BatteryData) {
        self.data = data;
        self.update().await
    }

    pub async fn set_voltage(&mut self, voltage: i32) {
        self.data.voltage = voltage;
        self.update().await
    }

    pub async fn set_capacity(&mut self, capacity: f64) {
        self.data.capacity = capacity;
        self.update().await
    }

    pub async fn set_current(&mut self, current: i32) {
        self.data.current = current;
        self.update().await
    }

    pub async fn set_temperature(&mut self, temperature: f64) {
        self.data.temperature = temperature;
        self.update().await
    }

    pub const fn voltage(&self) -> i32 {
        self.data.voltage
    }

    pub const fn capacity(&self) -> f64 {
        self.data.capacity
    }

    pub const fn current(&self) -> i32 {
        self.data.current
    }

    pub const fn temperature(&self) -> f64 {
        self.data.temperature
    }

    pub const fn data(&self) -> BatteryData {
        self.data
    }
}
