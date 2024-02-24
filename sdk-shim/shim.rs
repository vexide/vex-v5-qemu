#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = ".jump_table"]
pub static JUMP_TABLE: [Option<unsafe extern "C" fn()>; 0x1000] = {
    let mut table = [None; 0x1000];

    table[0x20 / 4] = Some(vexPrivateApiDisable as _);
    
    table
};

#[no_mangle]
pub unsafe extern "C" fn vexPrivateApiDisable() {
    loop { }
}