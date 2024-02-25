use std::{path::PathBuf, process::Command};

// TODO: fix this cursedness
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/armv7a-none-eabi/debug/kernel"
);

#[derive(clap::Parser)]
struct Opt {
    /// Whether to listen for a gdbstub connection.
    #[clap(short, long)]
    gdb: bool,

    /// The kernel binary to run. Defaults to the mock VEXos kernel.
    #[clap(short, long, default_value = DEFAULT_KERNEL)]
    kernel: PathBuf,

    /// The user code binary to be simulated.
    binary: PathBuf,
}

fn main() {
    let opt = <Opt as clap::Parser>::parse();

    let mut qemu = Command::new("qemu-system-arm")
        .args(&["-machine", "none"])
        .args(&["-cpu", "cortex-a7"])
        .args(&["-m", "256M"])
        .args(&[
            "-device",
            &format!("loader,file={},cpu-num=0", opt.kernel.display()),
        ])
        .args(&[
            "-device",
            &format!(
                "loader,file={},force-raw=on,addr=0x03800000",
                opt.binary.display()
            ),
        ])
        .args(&["-nographic"])
        .args(opt.gdb.then_some(["-S", "-s"]).unwrap_or_default())
        .spawn()
        .expect("Failed to start QEMU.");

    qemu.wait().expect("QEMU exited unexpectedly.");
}
