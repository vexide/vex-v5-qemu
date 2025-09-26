use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{sync::Mutex, task::AbortHandle, time::sleep};
use vex_v5_qemu_protocol::{
    distance_sensor::{DistanceObject, DistanceSensorData},
    SmartPortData,
};

use crate::peripherals::smartport::SmartPort;

#[derive(Debug)]
pub struct DistanceSensor {
    task: AbortHandle,
    data: Arc<Mutex<DistanceSensorData>>,
}

impl DistanceSensor {
    // TODO: determine this
    pub const UPDATE_INTERVAL: Duration = Duration::from_millis(10);

    pub fn new(mut port: SmartPort) -> Self {
        let start = Instant::now();
        let data = Arc::new(Mutex::new(DistanceSensorData {
            object: None,
            status: 0,
        }));

        Self {
            data: data.clone(),
            task: tokio::task::spawn(async move {
                loop {
                    port.send(
                        SmartPortData::DistanceSensor(*data.lock().await),
                        start.elapsed().as_millis() as u32,
                    )
                    .await;

                    sleep(Self::UPDATE_INTERVAL).await;
                }
            })
            .abort_handle(),
        }
    }

    pub async fn set_object(&mut self, object: Option<DistanceObject>) {
        self.data.lock().await.object = object;
    }

    pub async fn set_status(&mut self, status: u32) {
        self.data.lock().await.status = status;
    }
}

impl Drop for DistanceSensor {
    fn drop(&mut self) {
        self.task.abort();
    }
}
