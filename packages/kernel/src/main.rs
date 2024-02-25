#![no_std]
#![feature(c_variadic)]
#![allow(non_snake_case)]
#![no_main]

pub mod sdk;

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    #[link_name = "_cold_memory_start"]
    static COLD_MEMORY_START: *const ();

    #[link_name = "_vex_startup"]
    fn vex_startup() -> !;
}

extern "C" fn main() -> ! {
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
