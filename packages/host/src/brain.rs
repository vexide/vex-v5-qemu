use std::{
    io, option::Option, path::PathBuf, process::{ExitStatus, Stdio}, sync::Arc, time::Duration, vec::Vec
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::{
        mpsc::{self, Receiver, Sender},
        Barrier, Mutex, RwLock,
    },
    task::AbortHandle, time::sleep,
};
use vex_v5_qemu_protocol::{DisplayCommand, HostBoundPacket, KernelBoundPacket, SmartPortCommand};

use crate::{
    peripherals::{
        battery::Battery, display::Display, smartport::SmartPort, touch::Touchscreen, usb::Usb,
        Peripherals,
    },
};

#[derive(Debug, Clone)]
pub struct Binary {
    pub path: PathBuf,
    pub load_addr: u32,
}

pub struct Brain {
    pub peripherals: Option<Peripherals>,
    tx_task: AbortHandle,
    rx_task: AbortHandle,
    qemu: Arc<Mutex<Child>>,
}

impl Brain {
    #[allow(clippy::new_without_default)]
    pub fn new(
        mut qemu_command: Command,
        kernel: PathBuf,
        main_binary: Binary,
        linked_binary: Option<Binary>,
    ) -> io::Result<Self> {
        let link_addr: u32 = linked_binary.clone().map_or(0, |v| v.load_addr);
        let qemu_command = qemu_command
            .args(["-machine", "xilinx-zynq-a9,memory-backend=mem"])
            .args(["-cpu", "cortex-a9"])
            .args(["-object", "memory-backend-ram,id=mem,size=256M"])
            .args([
                "-device",
                &format!("loader,addr=0x200,data={},data-len=4,cpu-num=0", link_addr),
            ])
            .args([
                "-device",
                &format!("loader,file={},addr=0x100000,cpu-num=0", kernel.display()),
            ])
            .args([
                "-device",
                &format!(
                    "loader,file={},force-raw=on,addr={}",
                    main_binary.path.display(),
                    main_binary.load_addr
                ),
            ])
            .args(["-display", "none"])
            .args([
                "-semihosting",
                "-semihosting-config",
                "enable=on,target=native",
            ])
            .args(["-chardev", "stdio,id=uart"])
            .args(["-serial", "null"])
            .args(["-serial", "chardev:uart"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);

        if let Some(linked_binary) = linked_binary {
            qemu_command.arg("-device");
            qemu_command.arg(format!(
                "loader,file={},force-raw=on,addr={}",
                linked_binary.path.display(),
                linked_binary.load_addr
            ));
        }

        let (peripherals_tx, mut peripherals_rx) = mpsc::channel::<KernelBoundPacket>(1024);

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
        let (display_tx, display_rx) = mpsc::channel::<DisplayCommand>(1);

        let mut qemu = qemu_command.spawn()?;
        let mut qemu_stdin = qemu.stdin.take().unwrap();
        let mut qemu_stdout = qemu.stdout.take().unwrap();

        let qemu = Arc::new(Mutex::new(qemu));

        Ok(Self {
            qemu: qemu.clone(),
            tx_task: tokio::task::spawn(async move {
                loop {
                    if let Some(packet) = peripherals_rx.recv().await {
                        let encoded =
                            bincode::encode_to_vec(packet, bincode::config::standard()).unwrap();
                        let mut bytes = Vec::new();

                        bytes.extend((encoded.len() as u32).to_le_bytes());
                        bytes.extend(encoded);

                        qemu_stdin.write_all(&bytes).await.unwrap();
                    }
                }
            })
            .abort_handle(),
            rx_task: tokio::spawn(async move {
                let smartport_senders = [
                    port_1_tx, port_2_tx, port_3_tx, port_4_tx, port_5_tx, port_6_tx, port_7_tx,
                    port_8_tx, port_9_tx, port_10_tx, port_11_tx, port_12_tx, port_13_tx,
                    port_14_tx, port_15_tx, port_16_tx, port_17_tx, port_18_tx, port_19_tx,
                    port_20_tx, port_21_tx,
                ];

                loop {
                    let incoming_packet: HostBoundPacket = {
                        let packet_size = qemu_stdout.read_u32_le().await.unwrap() as usize;
                        let mut buf = vec![0u8; packet_size];

                        qemu_stdout.read_exact(&mut buf).await.unwrap();

                        bincode::decode_from_slice(&buf, bincode::config::standard())
                            .unwrap()
                            .0
                    };

                    match incoming_packet {
                        // Forward sent data to usb peripheral.
                        HostBoundPacket::UsbSerial(data) => {
                            _ = usb_tx.send(data).await;
                        }

                        // Kernel debugging stuff (logs mainly) goes to stderr
                        HostBoundPacket::KernelSerial(data) => {
                            let mut stderr = tokio::io::stderr();
                            stderr.write_all(&data).await.unwrap();
                            stderr.flush().await.unwrap();
                        }

                        // Doesn't matter for now.
                        HostBoundPacket::CodeSignature(_) => {}

                        // Kill QEMU child process when kernel requests exit.
                        HostBoundPacket::ExitRequest(code) => {
                            qemu.lock().await.kill().await.unwrap();
                            log::info!("Kernel exited with code {code}.");
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

                        HostBoundPacket::DisplayCommand { command } => {
                            _ = display_tx.send(command).await;
                        }
                    }
                }
            })
            .abort_handle(),
            peripherals: Some(Peripherals {
                battery: Battery::new(peripherals_tx.clone()),
                usb: Usb::new(peripherals_tx.clone(), usb_rx),

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

                display: Display::new(peripherals_tx.clone(), display_rx),
                touch: Touchscreen::new(peripherals_tx.clone()),
            }),
        })
    }

    pub async fn wait_for_exit(&mut self) -> io::Result<Option<ExitStatus>> {
        loop {
            if let Some(status) = self.qemu.lock().await.try_wait()? {
                return Ok(Some(status));
            }
            sleep(Duration::from_millis(10)).await;
        }
    }
}

impl Drop for Brain {
    fn drop(&mut self) {
        self.tx_task.abort();
        self.rx_task.abort();
    }
}
