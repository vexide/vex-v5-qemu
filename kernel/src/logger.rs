//! Kernel Logger Implementation

use alloc::format;
use log::{max_level, set_logger, set_max_level, LevelFilter, Log, Metadata, SetLoggerError};

use vex_v5_qemu_protocol::HostBoundPacket;

use crate::protocol;

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
            semihosting::eprintln!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
