use alloc::format;
use core::panic::PanicInfo;
use vex_v5_qemu_protocol::HostBoundPacket;

use crate::protocol::{self, exit};

#[panic_handler]
fn panic_handler(info: &PanicInfo<'_>) -> ! {
    semihosting::eprintln!("kernel {info}\n");

    exit(101);
}
