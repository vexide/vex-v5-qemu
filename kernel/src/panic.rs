#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    // reads as "kernel panicked at ..."
    semihosting::eprintln!("kernel {info}");
    semihosting::process::exit(101)
}
