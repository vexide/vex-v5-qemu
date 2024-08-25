use bincode::error::{DecodeError, EncodeError};
use miette::Diagnostic;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, ChildStdin, ChildStdout},
    sync::{mpsc::Receiver, Mutex}, task::JoinHandle,
};

use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket};

#[derive(Clone, Default, Debug)]
pub struct QemuConnection {
    pub inner: Arc<Mutex<Option<QemuConnectionInner>>>,
}

#[derive(Debug)]
pub struct QemuConnectionInner {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
    pub incoming_packets: Receiver<HostBoundPacket>,
    pub task: JoinHandle<()>,
}

#[derive(Error, Diagnostic, Debug)]
pub enum ConnectionError {
    #[error("Attempted to interact with a device while a program wasn't running.")]
    NotRunning,

    #[error(transparent)]
    #[diagnostic(code(simulator::io_error))]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(simulator::encode_error))]
    Encode(#[from] EncodeError),

    #[error(transparent)]
    #[diagnostic(code(simulator::decode_error))]
    Decode(#[from] DecodeError),
}

impl QemuConnection {
    pub async fn send_packet(&mut self, packet: KernelBoundPacket) -> Result<(), ConnectionError> {
        let mut guard = self.inner.lock().await;

        if let Some(qemu) = guard.as_mut() {
            let encoded = bincode::encode_to_vec(packet, bincode::config::standard())?;
            let mut bytes = Vec::new();

            bytes.extend((encoded.len() as u32).to_le_bytes());
            bytes.extend(encoded);

            qemu.stdin.write_all(&bytes).await?;

            Ok(())
        } else {
            Err(ConnectionError::NotRunning)
        }
    }
}
