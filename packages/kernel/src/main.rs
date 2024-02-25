#![no_std]
#![no_main]

pub mod sdk;

use core::{arch::global_asm, mem, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    #[link_name = "_vex_startup"]
    fn vex_startup() -> !;
}

extern "C" fn main() -> ! {
    unsafe {
        vex_startup();
    }

    panic!("VEX startup should not return!");
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
