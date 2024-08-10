use core::panic::PanicInfo;
use embedded_io::Write;
use lock_api::RawMutex;

use crate::{peripherals::UART1, utils::exit};

#[panic_handler]
fn panic_handler(info: &PanicInfo<'_>) -> ! {
    critical_section::with(|_| {
        unsafe {
            // SAFETY: The UART device will not be used after the panic has been printed
            // so we can consider the previous lock to be elapsed.
            UART1.raw().unlock();
        }

        // Reads as "kernel panicked at <...>:"
        writeln!(UART1.try_lock().unwrap(), "kernel {}", info).unwrap();

        exit(101);
    })
}
