use std::{option::Option, ffi::OsStr, fs::File, io::Read, path::PathBuf};

use anyhow::Context;
use clap_num::maybe_hex;
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use tokio::{
    io::{stdout, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};
use vex_v5_qemu_host::brain::Brain;

// TODO: fix this cursedness
const DEFAULT_KERNEL: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/armv7a-none-eabi/debug/kernel"
);

/// Simulate a VEX V5 robot program
#[derive(clap::Parser)]
struct Opt {
    /// Start the simulator in a paused state and open a GDB server.
    ///
    /// When enabled, the simulator will make a GDB server available on port
    /// 1234, allowing a debugger to set breakpoints in and step through the
    /// kernel or user code.
    #[clap(short, long, global = true, env = "V5_SIM_GDB")]
    gdb: bool,

    /// Override the kernel image.
    ///
    /// The simulator requires an emulated kernel to handle SDK calls
    /// and set up the virtual machine before running the robot code.
    /// This option defaults to a kernel designed to replicate the behavior
    /// of programs under VEXos.
    #[clap(short, long, global = true, default_value = DEFAULT_KERNEL, env = "V5_SIM_KERNEL_PATH")]
    kernel: PathBuf,

    /// Override the QEMU executable to a custom version of `qemu-system-arm`.
    #[clap(short, long, global = true, default_value = "qemu-system-arm", env = "V5_SIM_QEMU")]
    qemu: PathBuf,

    /// What type of program to simulate
    #[clap(subcommand)]
    program_type: Subcommands,

    /// Extra arguments to pass to QEMU.
    qemu_args: Vec<String>,
}

#[derive(clap::Subcommand)]
enum Subcommands {
    /// Simulate a monolith binary at <BIN>
    Monolith {
        /// The binary to run
        #[clap(env = "V5_SIM_BINARY_PATH")]
        bin: PathBuf,
    },
    /// Simulate a hot/cold binary at <HOT_BIN> and <COLD_BIN>
    HotCold {
        /// The hot binary to run
        #[clap(env = "V5_SIM_HOT_BINARY_PATH")]
        hot_bin: PathBuf,

        /// The cold binary to run
        #[clap(env = "V5_SIM_COLD_BINARY_PATH")]
        cold_bin: PathBuf,

        /// Override the address to load the hot binary at
        #[clap(long, global = true, default_value = "0x07800000", value_parser=maybe_hex::<u32>, env = "V5_SIM_HOT_ADDR")]
        hot_addr: u32,

        /// Override the address to load the cold binary at
        #[clap(long, global = true, default_value = "0x03800000", value_parser=maybe_hex::<u32>, env = "V5_SIM_COLD_ADDR")]
        cold_addr: u32,
    }
}

fn check_is_bin(mut file_path: PathBuf) {
    //file_path = file_path.canonicalize().unwrap();
    assert_eq!("bin", file_path.extension().unwrap_or(OsStr::new("bin")), "{} should have .bin extension", file_path.display());
    let mut buffer = [0 as u8; 4];
    let n = File::open(&file_path).expect(&*format!("{} does not exist", file_path.display())).read(&mut buffer[..]);
    let buffer = buffer;
    let elf_header = [0x7F as u8, 'E' as u8, 'L' as u8, 'F' as u8];
    assert_ne!(buffer, elf_header, "{} is an elf file, use a bin file instead", file_path.display());
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

    match opt.program_type {
        Subcommands::Monolith {bin} => {
            check_is_bin(bin.clone());
            brain
                .run_program(qemu, opt.kernel, bin, 0x03800000, None, None)
                .await
                .context("Failed to start QEMU.")?;
        }
        Subcommands::HotCold {hot_bin, cold_bin, hot_addr, cold_addr} => {
            check_is_bin(hot_bin.clone());
            check_is_bin(cold_bin.clone());
            brain
                .run_program(qemu, opt.kernel, hot_bin, hot_addr, Some(cold_bin), Some(cold_addr))
                .await
                .context("Failed to start QEMU.")?;
        }
    }

    // brain.kill_program().await.unwrap();

    let usb_task = tokio::task::spawn(async move {
        let mut usb = peripherals.usb;
        let mut out = stdout();

        loop {
            let mut buf = vec![0; 1024];
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
