use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use anyhow::Context;
use memmap2::{MmapOptions, MmapRaw};

mod ramfs;

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

fn main() -> anyhow::Result<()> {
    let opt = <Opt as clap::Parser>::parse();

    let ramfs = ramfs::RamFS::new().context("Failed to create in-memory filesystem.")?;
    let memory_file_path = dbg!(ramfs.path().join("v5-simulator"));

    let mut qemu = Command::new("qemu-system-arm")
        .args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
        .args(["-cpu", "cortex-a9"])
        .args([
            "-object",
            &format!(
                "memory-backend-file,id=mem,size=256M,mem-path={}",
                memory_file_path.display()
            ),
        ])
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
        .args([
            "-semihosting",
            "-semihosting-config",
            "enable=on,target=native",
        ])
        // .args(&[
        //     "-mon",
        //     "simmonitor,mode=control,pretty=on",
        //     "-chardev",
        //     "stdio,id=simmonitor,signal=off",
        // ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args(["-S"])
        .args(["-s"])
        .spawn()
        .context("Failed to start QEMU.")?;

    thread::sleep(std::time::Duration::from_millis(100));
    let memory_file = MmapOptions::new()
        .map_raw(
            &OpenOptions::new()
                .read(true)
                .write(true)
                .open(memory_file_path)
                .context("Failed to open memory file.")?,
        )
        .unwrap();

    let mut host_call_guest =
        unsafe { host_call::Guest::new_on_host(memory_file.as_mut_ptr().cast()) };
    let [mut call_cell, ..] = host_call_guest.take_call_cells().unwrap();

    loop {
        std::thread::sleep_ms(1000);
        // println!("Polling call cell...");
        call_cell = match call_cell.poll_incoming() {
            Ok(incoming) => match incoming.call {
                host_call::Call::Write { data, written } => {
                    dbg!(data);
                    *written = 0xdeadbeef;
                    incoming.cell.complete()
                }
            },
            Err(call_cell) => call_cell,
        }
    }

    qemu.wait().context("QEMU exited unexpectedly.")?;
    drop(ramfs);

    // TODO: clean up temp files

    Ok(())
}
