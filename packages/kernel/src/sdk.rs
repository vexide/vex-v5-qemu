use core::ptr;

#[no_mangle]
#[link_section = ".jump_table"]
pub static mut JUMP_TABLE: [*const (); 0x1000] = {
    let mut table = [ptr::null(); 0x1000];

    table[0x20 / 4] = vex_private_api_disable as _;

    table
};

#[no_mangle]
unsafe extern "C" fn vex_private_api_disable() {}
