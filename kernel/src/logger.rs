//! Kernel Logger Implementation

use core::time::Duration;

use alloc::format;
use log::{max_level, set_logger, set_max_level, LevelFilter, Log, Metadata, SetLoggerError};

use vex_v5_qemu_protocol::HostBoundPacket;

use crate::{protocol, sdk::vexSystemHighResTimeGet};

const ESCAPES: [Option<&str>; 6] = [
    None,             // Default foreground
    Some("\x1B[31m"), // Error (red)
    Some("\x1B[33m"), // Warn (yellow)
    Some("\x1B[34m"), // Info (blue)
    Some("\x1B[36m"), // Debug (cyan)
    Some("\x1B[37m"), // Trace (white)
];

pub struct KernelLogger;

impl KernelLogger {
    pub fn init(&'static self, level: LevelFilter) -> Result<(), SetLoggerError> {
        set_logger(self)?;
        set_max_level(level);

        Ok(())
    }
}

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            let timestamp = Duration::from_micros(vexSystemHighResTimeGet());
            let mins = timestamp.as_secs() / 60;
            let submin_secs = timestamp.as_millis() % 60;

            protocol::send_packet(HostBoundPacket::KernelSerial(
                format!(
                    "{:02}:{:02}:{:02} {}[{}]\x1B[0m kernel: {}\n",
                    mins,
                    submin_secs,
                    timestamp.as_millis(),
                    ESCAPES[record.level() as usize].unwrap_or_default(),
                    record.level(),
                    record.args()
                )
                .as_bytes()
                .to_vec(),
            ))
            .unwrap();
        }
    }

    fn flush(&self) {}
}
