use tokio::sync::mpsc::{Receiver, Sender};
use vex_v5_qemu_protocol::{KernelBoundPacket, SmartPortCommand, SmartPortData};

#[derive(Debug)]
pub struct SmartPort {
    index: u8,
    tx: Sender<KernelBoundPacket>,
    rx: Receiver<SmartPortCommand>,
}

impl SmartPort {
    pub(crate) const fn new(
        index: u8,
        tx: Sender<KernelBoundPacket>,
        rx: Receiver<SmartPortCommand>,
    ) -> Self {
        Self { index, tx, rx }
    }

    pub const fn index(&self) -> u8 {
        self.index
    }

    pub async fn send(&mut self, data: SmartPortData, timestamp: u32) {
        // Safe to unwrap since we are dropped at the same time as the receiver.
        self.tx.send(KernelBoundPacket::SmartPortUpdate {
            port_index: self.index,
            data,
            timestamp,
        }).await.unwrap()
    }

    pub async fn recv(&mut self) -> SmartPortCommand {
        // Safe to unwrap since we are dropped at the same time as the sender.
        self.rx.recv().await.unwrap()
    }
}
