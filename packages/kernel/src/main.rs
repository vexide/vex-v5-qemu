#![no_std]
#![no_main]
#![feature(c_variadic)]

pub mod sdk;

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::hint::black_box(_info);
    loop {}
}

extern "C" {
    #[link_name = "_cold_memory_start"]
    static COLD_MEMORY_START: *const ();

    #[link_name = "_vex_startup"]
    fn vex_startup();
}

extern "C" fn main() -> ! {
    unsafe {
        let mut call_cell_guest = host_call::Guest::new_on_guest();
        let [call_cell, ..] = call_cell_guest.take_call_cells().unwrap();
        
        let mut written = 0;

        let call_cell = call_cell.perform(host_call::Call::Write {
            data: "Hello, World!".as_bytes(),
            written: &mut written,
        });

        loop {}
    }

    unsafe {
        vex_startup();
    }

    unreachable!("VEX startup should not return!");
}

global_asm!(
    r#"
        .section .text
        .global _start
        .type _start, STT_FUNC

    _start:
        ldr sp, =0x10000
        bl {main}
    "#,
    main = sym main
);
