use std::{
    io::{stderr, stdout, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Context;
use log::{debug, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket};

pub mod protocol;

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
    /// When enabled, the simulator will make a GDB server available on port
    /// 1234, allowing a debugger to set breakpoints in and step through the
    /// kernel or user code.
    #[clap(short, long, env = "V5_SIM_GDB")]
    gdb: bool,

    /// Override the kernel image.
    ///
    /// The simulator requires an emulated kernel to handle SDK calls
    /// and set up the virtual machine before running the robot code.
    /// This option defaults to a kernel designed to replicate the behavior
    /// of programs under VEXos.
    #[clap(short, long, default_value = DEFAULT_KERNEL, env = "V5_SIM_KERNEL_PATH")]
    kernel: PathBuf,

    /// Override the QEMU executable to a custom version of `qemu-system-arm`.
    #[clap(short, long, default_value = "qemu-system-arm", env = "V5_SIM_QEMU")]
    qemu: PathBuf,

    #[clap(env = "V5_SIM_BINARY_PATH")]
    binary: PathBuf,

    /// Extra arguments to pass to QEMU.
    qemu_args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = <Opt as clap::Parser>::parse();

    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_thread_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut qemu = Command::new(opt.qemu);
    qemu.args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
        .args(["-cpu", "cortex-a9"])
        .args(["-object", "memory-backend-ram,id=mem,size=256M"])
        .args([
            "-device",
            &format!(
                "loader,file={},addr=0x100000,cpu-num=0",
                opt.kernel.display()
            ),
        ])
        .args([
            "-device",
            &format!(
                "loader,file={},force-raw=on,addr=0x03800000",
                opt.binary.display()
            ),
        ])
        .args(["-display", "none"])
        .args(["-chardev", "stdio,id=char0"])
        .args(["-serial", "null"])
        .args(["-serial", "chardev:char0"])
        .args(opt.qemu_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    if opt.gdb {
        qemu.args(["-S", "-s"]);
    }
    let mut qemu = qemu.spawn().context("Failed to start QEMU.")?;

    let mut qemu_stdout = qemu.stdout.take().unwrap();
    let mut qemu_stdin = qemu.stdin.take().unwrap();

    while let Ok(packet) = protocol::recv_packet(&mut qemu_stdout) {
        if let Some(packet) = &packet {
            match packet {
                HostBoundPacket::UserSerial(data) => {
                    let mut stdout = stdout().lock();

                    stdout.write_all(data).unwrap();
                    stdout.flush().unwrap();
                }
                HostBoundPacket::KernelSerial(data) => {
                    let mut stderr = stderr().lock();
                    stderr.write_all(data).unwrap();
                    stderr.flush().unwrap();
                }
                HostBoundPacket::CodeSignature(sig) => {
                    debug!("Received code signature: {:?}", sig);
                }
                HostBoundPacket::ExitRequest(code) => {
                    qemu.kill().unwrap();
                    std::process::exit(*code);
                }
                _ => {},
            }
        }
    }

    qemu.wait().context("QEMU exited unexpectedly.")?;

    Ok(())
}
