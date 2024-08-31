use bincode::error::{DecodeError, EncodeError};
use miette::Diagnostic;
use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, ChildStdout},
};
use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket};

#[derive(Debug)]
pub struct QemuConnection {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
}

#[derive(Error, Diagnostic, Debug)]
pub enum ConnectionError {
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
        let encoded = bincode::encode_to_vec(packet, bincode::config::standard())?;
        let mut bytes = Vec::new();

        bytes.extend((encoded.len() as u32).to_le_bytes());
        bytes.extend(encoded);

        self.stdin.write_all(&bytes).await?;

        Ok(())
    }

    pub async fn recv_packet(&mut self) -> Result<HostBoundPacket, ConnectionError> {
        let packet_size = self.stdout.read_u32_le().await.unwrap() as usize;
        let mut buf = vec![0; packet_size];
        self.stdout.read_exact(&mut buf).await.unwrap();

        Ok(bincode::decode_from_slice(&buf, bincode::config::standard())
            .unwrap()
            .0)
    }
}
