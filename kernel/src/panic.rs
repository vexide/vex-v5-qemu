use alloc::format;
use core::panic::PanicInfo;
use vex_v5_qemu_protocol::HostBoundPacket;

use crate::protocol::{self, exit};

#[panic_handler]
fn panic_handler(info: &PanicInfo<'_>) -> ! {
    protocol::send_packet(HostBoundPacket::KernelSerial(
        format!("kernel {info}\n").as_bytes().to_vec(),
    ))
    .ok();

    exit(101);
}
