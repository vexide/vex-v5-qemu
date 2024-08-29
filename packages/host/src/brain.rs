use std::{
    io,
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::Arc,
};

use tokio::{
    io::AsyncWriteExt,
    process::Command,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    task::AbortHandle,
};
use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket, SmartPortCommand};

use crate::{
    connection::QemuConnection,
    peripherals::{battery::Battery, smartport::SmartPort, usb::Usb, Peripherals},
};

#[derive(Debug)]
pub struct Brain {
    pub peripherals: Option<Peripherals>,
    connection: Arc<Mutex<Option<QemuConnection>>>,
    task: AbortHandle,
}

impl Brain {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let connection = Arc::new(Mutex::new(None));

        let (peripherals_tx, peripherals_rx) = mpsc::channel::<KernelBoundPacket>(1024);

        // Each of these channels represents a serial line for device commands from the
        // kernel to a smartport. Commands sent to devices by the kernel are
        // forwarded by the brain's packet event loop task as described later.
        let (port_1_tx, port_1_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_2_tx, port_2_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_3_tx, port_3_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_4_tx, port_4_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_5_tx, port_5_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_6_tx, port_6_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_7_tx, port_7_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_8_tx, port_8_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_9_tx, port_9_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_10_tx, port_10_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_11_tx, port_11_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_12_tx, port_12_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_13_tx, port_13_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_14_tx, port_14_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_15_tx, port_15_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_16_tx, port_16_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_17_tx, port_17_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_18_tx, port_18_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_19_tx, port_19_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_20_tx, port_20_rx) = mpsc::channel::<SmartPortCommand>(1);
        let (port_21_tx, port_21_rx) = mpsc::channel::<SmartPortCommand>(1);

        let (usb_tx, usb_rx) = mpsc::channel::<Vec<u8>>(1);

        Self {
            connection: connection.clone(),
            task: tokio::task::spawn(async move {
                let mut peripherals_rx = peripherals_rx;
                let smartport_senders: [Sender<SmartPortCommand>; 21] = [
                    port_1_tx, port_2_tx, port_3_tx, port_4_tx, port_5_tx, port_6_tx, port_7_tx,
                    port_8_tx, port_9_tx, port_10_tx, port_11_tx, port_12_tx, port_13_tx,
                    port_14_tx, port_15_tx, port_16_tx, port_17_tx, port_18_tx, port_19_tx,
                    port_20_tx, port_21_tx,
                ];

                // This is the event loop that facilitates packet exchange with the QEMU
                // process over the simulator protocol. It has (roughly) two jobs:
                // - Receive packets from peripherals and send them to the kernel.
                // - Receive packets from the kernel and forward them to peripherals.
                loop {
                    // Receive incoming packets from peripherals.
                    let peripherals_packet = peripherals_rx.try_recv().ok();

                    if let Some(connection) = connection.lock().await.as_mut() {
                        // Send the latest packet from peripherals to the kernel.
                        if let Some(peripherals_packet) = peripherals_packet {
                            connection.send_packet(peripherals_packet).await.unwrap();
                        }

                        // If a child process is running, then receive kernel packets and forward
                        // them to a respective peripheral's `Receiver`.
                        match connection.recv_packet().await.unwrap() {
                            // TODO: Forward this to a `Usb` peripheral once that exists.
                            HostBoundPacket::UserSerial(data) => {
                                log::debug!("Received user serial data");
                                usb_tx.send(data).await.unwrap();
                            }

                            // I'm not sure if this should be handled as part of the USB
                            // peripheral in the future or not, since it's kernel log output
                            // and panic messages.
                            HostBoundPacket::KernelSerial(data) => {
                                let mut stderr = tokio::io::stderr();
                                stderr.write_all(&data).await.unwrap();
                                stderr.flush().await.unwrap();
                            }

                            // Doesn't matter for now.
                            HostBoundPacket::CodeSignature(_) => {}

                            // Kill QEMU child process when kernel requests exit.
                            HostBoundPacket::ExitRequest(code) => {
                                log::info!("Kernel exited with code {code}.");
                                connection.child.kill().await.unwrap();
                            }

                            // The kernel has sent a device command packet to a specific smartport,
                            // so we must forward that packet to the respective smartport's
                            // receiver.
                            HostBoundPacket::SmartPortCommand { port, command } => {
                                if let Some(port_tx) = smartport_senders.get(port as usize) {
                                    // We ignore errors if the packet send fails, since it means
                                    // the user has dropped the smartport and by extension the
                                    // receiver, meaning there's nothing to send to.
                                    _ = port_tx.send(command).await;
                                }
                            }

                            // Not implemented yet.
                            _ => {}
                        }
                    }
                }
            })
            .abort_handle(),
            peripherals: Some(Peripherals {
                battery: Battery::new(peripherals_tx.clone()),

                port_1: SmartPort::new(0, peripherals_tx.clone(), port_1_rx),
                port_2: SmartPort::new(1, peripherals_tx.clone(), port_2_rx),
                port_3: SmartPort::new(2, peripherals_tx.clone(), port_3_rx),
                port_4: SmartPort::new(3, peripherals_tx.clone(), port_4_rx),
                port_5: SmartPort::new(4, peripherals_tx.clone(), port_5_rx),
                port_6: SmartPort::new(5, peripherals_tx.clone(), port_6_rx),
                port_7: SmartPort::new(6, peripherals_tx.clone(), port_7_rx),
                port_8: SmartPort::new(7, peripherals_tx.clone(), port_8_rx),
                port_9: SmartPort::new(8, peripherals_tx.clone(), port_9_rx),
                port_10: SmartPort::new(9, peripherals_tx.clone(), port_10_rx),
                port_11: SmartPort::new(10, peripherals_tx.clone(), port_11_rx),
                port_12: SmartPort::new(11, peripherals_tx.clone(), port_12_rx),
                port_13: SmartPort::new(12, peripherals_tx.clone(), port_13_rx),
                port_14: SmartPort::new(13, peripherals_tx.clone(), port_14_rx),
                port_15: SmartPort::new(14, peripherals_tx.clone(), port_15_rx),
                port_16: SmartPort::new(15, peripherals_tx.clone(), port_16_rx),
                port_17: SmartPort::new(16, peripherals_tx.clone(), port_17_rx),
                port_18: SmartPort::new(17, peripherals_tx.clone(), port_18_rx),
                port_19: SmartPort::new(18, peripherals_tx.clone(), port_19_rx),
                port_20: SmartPort::new(19, peripherals_tx.clone(), port_20_rx),
                port_21: SmartPort::new(20, peripherals_tx.clone(), port_21_rx),
                usb: Usb::new(usb_rx),
            }),
        }
    }

    pub async fn run_program(
        &mut self,
        mut qemu_command: Command,
        kernel: PathBuf,
        binary: PathBuf,
    ) -> io::Result<()> {
        qemu_command
            .args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
            .args(["-cpu", "cortex-a9"])
            .args(["-object", "memory-backend-ram,id=mem,size=256M"])
            .args([
                "-device",
                &format!("loader,file={},addr=0x100000,cpu-num=0", kernel.display()),
            ])
            .args([
                "-device",
                &format!(
                    "loader,file={},force-raw=on,addr=0x03800000",
                    binary.display()
                ),
            ])
            .args(["-display", "none"])
            .args(["-chardev", "stdio,id=char0"])
            .args(["-serial", "null"])
            .args(["-serial", "chardev:char0"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);

        let mut child = qemu_command.spawn()?;

        *self.connection.lock().await = Some(QemuConnection {
            stdin: child.stdin.take().unwrap(),
            stdout: child.stdout.take().unwrap(),
            child,
        });

        Ok(())
    }

    pub async fn wait_for_exit(&mut self) -> io::Result<Option<ExitStatus>> {
        while let Some(connection) = self.connection.lock().await.as_mut() {
            if let Some(status) = connection.child.try_wait()? {
                return Ok(Some(status));
            }
        }
        Ok(None)
    }
}

impl Drop for Brain {
    fn drop(&mut self) {
        self.task.abort();
    }
}
