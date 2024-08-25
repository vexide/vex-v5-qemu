use std::{
    io::{stderr, stdout, Write},
    path::PathBuf,
};

use anyhow::Context;
use log::{debug, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use tokio::process::Command;
use vex_v5_qemu_host::brain::Brain;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let mut brain = Brain::new();
    let mut peripherals = brain.peripherals.take().unwrap();

    let mut qemu = Command::new("qemu-system-arm");
    if opt.gdb {
        qemu.args(["-S", "-s"]);
    }

    brain
        .run_program(qemu, opt.kernel, opt.binary)
        .await
        .context("Failed to start QEMU.")?;
    brain.wait_for_exit().await?;

    Ok(())
}
