use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc::Sender, Mutex, Notify},
    task::AbortHandle,
    time::sleep,
};
use vex_v5_qemu_protocol::{
    geometry::Point2,
    touch::{TouchData, TouchEvent},
    KernelBoundPacket,
};

#[derive(Debug)]
pub struct Touchscreen {
    data: Arc<Mutex<TouchData>>,
    notify: Arc<Notify>,
    task: AbortHandle,
}

impl Touchscreen {
    pub const UPDATE_INTERVAL: Duration = Duration::from_millis(5);

    pub fn new(tx: Sender<KernelBoundPacket>) -> Self {
        let data = Arc::new(Mutex::new(TouchData::default()));
        let notify = Arc::new(Notify::new());

        Self {
            data: data.clone(),
            notify: notify.clone(),
            task: tokio::task::spawn(async move {
                loop {
                    notify.notified().await;

                    let data = *data.lock().await;
                    tx.send(KernelBoundPacket::Touch(data)).await.unwrap();

                    sleep(Self::UPDATE_INTERVAL).await;
                }
            }).abort_handle(),
        }
    }

    pub async fn set_data(&mut self, data: TouchData) {
        *self.data.lock().await = data;
        self.notify.notify_one();
    }

    pub async fn set_point(&mut self, point: Point2<i16>) {
        self.data.lock().await.point = point;
        self.notify.notify_one();
    }

    pub async fn set_event(&mut self, event: TouchEvent) {
        self.data.lock().await.event = event;
        self.notify.notify_one();
    }
}

impl Drop for Touchscreen {
    fn drop(&mut self) {
        self.task.abort();
    }
}
