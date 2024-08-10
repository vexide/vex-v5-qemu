//! Kernel Logger Implementation

use log::{max_level, set_logger, set_max_level, LevelFilter, Log, Metadata, SetLoggerError};

use embedded_io::Write;

use crate::peripherals::UART1;

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
            writeln!(UART1.lock(), "[{}] {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}
