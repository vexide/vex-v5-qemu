use std::{io, path::PathBuf, process::Stdio};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::mpsc,
};
use vex_v5_qemu_protocol::{battery::BatteryData, HostBoundPacket};

use crate::{
    connection::{QemuConnection, QemuConnectionInner},
    peripherals::{battery::Battery, Peripherals},
};

#[derive(Default, Debug)]
pub struct Brain {
    connection: QemuConnection,
    pub peripherals: Option<Peripherals>,
}

impl Brain {
    pub fn new() -> Self {
        let connection = QemuConnection::default();
        Self {
            connection: connection.clone(),
            peripherals: Some(Peripherals {
                battery: Battery::new(connection.clone(), BatteryData::default()),
            }),
        }
    }

    pub async fn run_program(
        &mut self,
        mut qemu_command: Command,
        program: PathBuf,
        kernel: PathBuf,
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
                    program.display()
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

        let (tx, rx) = mpsc::channel(1);

        let mut child = qemu_command.spawn()?;
        *self.connection.inner.lock().await = Some(QemuConnectionInner {
            stdin: child.stdin.take().unwrap(),
            stdout: child.stdout.take().unwrap(),
            child,
            incoming_packets: rx,
        });

        tokio::task::spawn({
            let connection = self.connection.clone();
            async move {
                loop {
                    let mut inner = connection.inner.lock().await;
                    let stdout = &mut inner.as_mut().unwrap().stdout;

                    let packet_size = stdout.read_u32_le().await.unwrap() as usize;
                    let mut buf = vec![0; packet_size];
                    stdout.read_exact(&mut buf).await.unwrap();

                    let packet = bincode::decode_from_slice(&buf, bincode::config::standard())
                        .unwrap()
                        .0;

                    match packet {
                        HostBoundPacket::UserSerial(data) => {
                            let mut stdout = tokio::io::stdout();

                            stdout.write_all(&data).await.unwrap();
                            stdout.flush().await.unwrap();
                        }
                        HostBoundPacket::KernelSerial(data) => {
                            let mut stderr = tokio::io::stderr();

                            stderr.write_all(&data).await.unwrap();
                            stderr.flush().await.unwrap();
                        }
                        HostBoundPacket::ExitRequest(code) => {
                            log::info!("Kernel exited with code {code}.");
                            inner.as_mut().unwrap().child.kill().await.unwrap();
                        }
                        _ => tx.send(packet).await.unwrap(),
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn wait_for_exit(&mut self) -> io::Result<()> {
        if let Some(connection) = self.connection.inner.lock().await.as_mut() {
            connection.child.wait().await?;
        }

        Ok(())
    }
}
