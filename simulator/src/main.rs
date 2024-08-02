use std::{
    path::PathBuf, process::{Command, Stdio},
};

use anyhow::Context;
use vex_v5_qemu_protocol::HostBoundPacket;

mod protocol;

// TODO: fix this cursedness
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../kernel/target/armv7a-none-eabi/debug/kernel"
);

/// Simulate the VEX V5 robot program at <BINARY>.
#[derive(clap::Parser)]
struct Opt {
    /// Start the simulator in a paused state and open a GDB server.
    ///
    /// When enabled, the simulator will make a GDB server available on port 1234,
    /// allowing a debugger to set breakpoints in and step through the kernel or user code.
    #[clap(short, long)]
    gdb: bool,

    /// Override the kernel image.
    ///
    /// The simulator requires an emulated kernel to handle SDK calls
    /// and set up the virtual machine before running the robot code.
    /// This option defaults to a kernel designed to replicate the behavior
    /// of programs under VEXos.
    #[clap(short, long, default_value = DEFAULT_KERNEL)]
    kernel: PathBuf,

    /// Override the QEMU executable to a custom version of `qemu-system-arm`.
    #[clap(short, long, default_value = "qemu-system-arm")]
    qemu: PathBuf,

    binary: PathBuf,

    /// Extra arguments to pass to QEMU.
    qemu_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = <Opt as clap::Parser>::parse();

    let mut qemu = Command::new(opt.qemu);
    qemu.args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
        .args(["-cpu", "cortex-a9"])
        .args(["-object", "memory-backend-ram,id=mem,size=256M"])
        .args([
            "-device",
            &format!("loader,file={},cpu-num=0", opt.kernel.display()),
        ])
        .args([
            "-device",
            &format!(
                "loader,file={},force-raw=on,addr=0x03800000",
                opt.binary.display()
            ),
        ])
        .args(["-nographic"])
        .args([
            // Semihosting interface allows host <-> guest communication
            "-semihosting",
            "-semihosting-config",
            "enable=on,target=native",
        ])
        .args(opt.qemu_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    if opt.gdb {
        qemu.args(["-S", "-s"]);
    }
    let mut qemu = qemu.spawn().context("Failed to start QEMU.")?;

    let mut stdout = qemu.stdout.take().unwrap();
    let mut stdin = qemu.stdin.take().unwrap();

    while protocol::recv_packet(&mut stdout).expect("Failed to recieve handshake packet from guest.")
        != Some(HostBoundPacket::Handshake)
    {
        core::hint::spin_loop();
    }

    qemu.wait().context("QEMU exited unexpectedly.")?;

    Ok(())
}
