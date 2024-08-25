use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, State};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tokio::{
    io::{stderr, AsyncWriteExt},
    sync::Mutex,
};
use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket};

use crate::{
    protocol::{self, DisplayClearPayload, DisplayDrawPayload, DisplayScrollPayload},
    AppState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QemuOptions {
    gdb: bool,
    kernel: PathBuf,
    qemu: PathBuf,
    binary: PathBuf,
    qemu_args: Vec<String>,
}

/// Kills the currently running QEMU subprocess.
#[tauri::command]
pub fn kill_qemu(state: State<'_, Mutex<AppState>>) {
    let mut state = state.blocking_lock();

    if let Some(qemu_process) = state.qemu_process.take() {
        qemu_process.kill().ok();
    }
}

/// Spawns a new QEMU subprocess.
#[tauri::command]
pub fn spawn_qemu(state: State<'_, Mutex<AppState>>, app: tauri::AppHandle, opts: QemuOptions) {
    let mut qemu = app
        .shell()
        .command(opts.qemu) // TODO: make this a sidecar
        .set_raw_out(true) // Otherwise stdio events will only fire after newlines
        .args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
        .args(["-cpu", "cortex-a9"])
        .args(["-object", "memory-backend-ram,id=mem,size=256M"])
        .args([
            "-device",
            &format!(
                "loader,file={},addr=0x100000,cpu-num=0",
                opts.kernel.display()
            ),
        ])
        .args([
            "-device",
            &format!(
                "loader,file={},force-raw=on,addr=0x03800000",
                opts.binary.display()
            ),
        ])
        .args(["-display", "none"])
        .args(["-chardev", "stdio,id=char0"])
        .args(["-serial", "null"])
        .args(["-serial", "chardev:char0"])
        .args(opts.qemu_args);

    if opts.gdb {
        qemu = qemu.args(["-S", "-s"]);
    }

    let (mut rx, child) = qemu.spawn().expect("Failed to spawn QEMU");

    state.blocking_lock().qemu_process = Some(child);

    tauri::async_runtime::spawn(async move {
        let state = app.state::<Mutex<AppState>>();

        // Temporary buffer for either the packet length or the packet contents.
        let mut stdout_buffer: Vec<u8> = Vec::new();
        // Length of the next packet. This is `None` while the length is still being
        // read.
        let mut next_packet_len: Option<u32> = None;

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    stdout_buffer.extend(bytes);

                    // Read the length of the next packet.
                    if next_packet_len.is_none() {
                        if stdout_buffer.len() < std::mem::size_of::<u32>() {
                            // In the event that we haven't read enough bytes to reach
                            // the length's size, then just continue waiting for more
                            // until we get 4 bytes to read.
                            continue;
                        }

                        // We have now read 4 bytes off of stdout, so we can safely decode bytes 0-4
                        // in stdout into a u32 representing the length.
                        next_packet_len = Some(u32::from_le_bytes(
                            stdout_buffer[0..std::mem::size_of::<u32>()]
                                .try_into()
                                .unwrap(),
                        ));
                        stdout_buffer.drain(0..4); // Wipe the length from the
                                                   // temporary read buffer.
                    };

                    // Read from stdout until we have read the full packet length.
                    let packet_len = next_packet_len.unwrap() as usize;
                    if stdout_buffer.len() < packet_len {
                        continue;
                    }

                    // Decode the packet and respond accordingly.
                    match bincode::decode_from_slice(&stdout_buffer, bincode::config::standard())
                        .unwrap()
                        .0
                    {
                        HostBoundPacket::UserSerial(data) => {
                            app.emit("user_serial", data).unwrap();
                        }
                        HostBoundPacket::KernelSerial(data) => {
                            app.emit("kernel_serial", data.clone()).unwrap();
                            let mut stderr = stderr();
                            stderr.write_all(&data).await.unwrap();
                            stderr.flush().await.unwrap();
                        }
                        HostBoundPacket::CodeSignature(sig) => {
                            log::debug!("Received code signature: {:?}", sig);
                        }
                        HostBoundPacket::ExitRequest(code) => {
                            let process = state.lock().await.qemu_process.take().unwrap();
                            process.kill().unwrap();
                            log::debug!("Exit request completed. Code {code}.");
                        }
                        HostBoundPacket::DisplayDraw {
                            command,
                            color,
                            clip_region,
                        } => {
                            app.emit(
                                "display_draw",
                                DisplayDrawPayload {
                                    command,
                                    color,
                                    clip_region,
                                },
                            )
                            .unwrap();
                        }
                        HostBoundPacket::DisplayErase { color, clip_region } => {
                            app.emit("display_clear", DisplayClearPayload { color, clip_region })
                                .unwrap();
                        }
                        HostBoundPacket::DisplayScroll {
                            location,
                            lines,
                            background,
                            clip_region,
                        } => {
                            app.emit(
                                "display_scroll",
                                DisplayScrollPayload {
                                    location,
                                    lines,
                                    background,
                                    clip_region,
                                },
                            )
                            .unwrap();
                        }
                        HostBoundPacket::DisplayRender => {
                            app.emit("display_render", ()).unwrap();
                        }
                        HostBoundPacket::DisplayRenderMode(mode) => {
                            app.emit("display_set_render_mode", mode).unwrap();
                        }
                    }

                    // Wipe the bytes we just decoded from the temporary buffer.
                    stdout_buffer.drain(0..packet_len);
                    // At this point we are ready to read the next packet's length.
                    next_packet_len = None;
                }
                CommandEvent::Stderr(bytes) => {
                    let mut stderr = stderr();
                    stderr.write_all(&bytes).await.unwrap();
                    stderr.flush().await.unwrap();
                }
                CommandEvent::Terminated(_payload) => {
                    log::info!("QEMU has terminated.");
                    return;
                }
                _ => log::debug!("Received child process event {:?}", event),
            };
        }
    });
}
