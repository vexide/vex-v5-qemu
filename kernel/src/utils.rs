use log::info;

pub fn exit(code: i32) -> ! {
    info!("Exiting with code {}", code);

    // not implemented yet
    loop {
        core::hint::spin_loop();
    }
}
