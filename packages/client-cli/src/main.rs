use std::{option::Option, path::PathBuf};

use anyhow::Context;
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use tokio::{
    io::{stdout, AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use vex_v5_qemu_host::brain::{Binary, Brain};

#[cfg(debug_assertions)]
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../kernel/target/armv7a-none-eabi/debug/kernel"
);

#[cfg(not(debug_assertions))]
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../kernel/target/armv7a-none-eabi/release/kernel"
);

/// Simulate a VEX V5 robot program
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

    /// Main program binary
    #[clap(long)]
    program: PathBuf,

    /// Main program binary load address
    #[clap(long)]
    load_addr: Option<u32>,

    /// Linked program binary
    #[clap(long, requires("link_addr"))]
    link: Option<PathBuf>,

    /// Linked program binary load address
    #[clap(long, value_parser = validate_address_range)]
    link_addr: Option<u32>,

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
    let peripherals = brain.peripherals.take().unwrap();

    let mut qemu = Command::new("qemu-system-arm");
    if opt.gdb {
        qemu.args(["-S", "-s"]);
    }

    qemu.args(opt.qemu_args);

    brain
        .run_program(
            qemu,
            opt.kernel,
            Binary {
                path: opt.program,
                load_addr: opt.load_addr.unwrap_or(0x03800000),
            },
            opt.link.map(|link| Binary {
                path: link,
                load_addr: opt.link_addr.unwrap(),
            }),
        )
        .await
        .context("Failed to start QEMU.")?;

    let usb_task = tokio::task::spawn(async move {
        let mut usb = peripherals.usb;
        let mut out = stdout();

        loop {
            let mut buf = vec![0; 2048];
            let n = usb.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }

            out.write_all(&buf[..n]).await.unwrap();
            out.flush().await.unwrap();
        }
    });

    brain.wait_for_exit().await?;
    usb_task.abort();

    Ok(())
}

fn validate_address_range(s: &str) -> Result<u32, String> {
    clap_num::maybe_hex_range(s, 0x03800000, 0x8000000)
}
