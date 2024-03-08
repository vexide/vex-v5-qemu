use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

use memmap2::{MmapOptions, MmapRaw};

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
        .args(&["-machine", "none,memory-backend=mem"])
        .args(&["-cpu", "cortex-a7"])
        .args(&[
            "-object",
            "memory-backend-file,id=mem,size=256M,mem-path=/dev/shm/v5-simulator",
        ])
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
        // .args(&[
        //     "-mon",
        //     "simmonitor,mode=control,pretty=on",
        //     "-chardev",
        //     "stdio,id=simmonitor,signal=off",
        // ])
        // .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .args(&["-nographic", "-S"])
        .spawn()
        .expect("Failed to start QEMU.");

    thread::sleep(std::time::Duration::from_millis(100));
    let memory_file = MmapOptions::new()
        .map_raw(
            &OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/shm/v5-simulator")
                .expect("Failed to open memory file."),
        )
        .unwrap();

    let mut host_call_guest =
        unsafe { host_call::Guest::new_on_host(memory_file.as_mut_ptr().cast()) };
    let [mut call_cell, ..] = host_call_guest.take_call_cells().unwrap();

    loop {
        std::thread::sleep_ms(1000);
        println!("Polling call cell...");
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

    qemu.wait().expect("QEMU exited unexpectedly.");

    // TODO: clean up temp files
}
