use std::{io::{ErrorKind, Read, Write}, process::{ChildStdin, ChildStdout}};

use thiserror::Error;
use miette::Diagnostic;
use bincode::error::{DecodeError, EncodeError};
use vex_v5_qemu_protocol::{GuestBoundPacket, HostBoundPacket};

#[derive(Error, Diagnostic, Debug)]
pub enum ProtocolError {
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

pub fn send_packet(stdin: &mut ChildStdin, packet: GuestBoundPacket) -> Result<(), ProtocolError> {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard())?;

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    stdin.write_all(&bytes)?;

    Ok(())
}

pub fn recv_packet(stdout: &mut ChildStdout) -> Result<Option<HostBoundPacket>, ProtocolError> {
    // Read packet length
    let len = {
        let mut buf = [0; 4];

        if let Err(err) = stdout.read_exact(&mut buf) {
            if err.kind() == ErrorKind::UnexpectedEof {
                return Ok(None);
            } else {
                return Err(ProtocolError::Io(err));
            }
        }

        u32::from_le_bytes(buf)
    };

    // Read packet payload
    let mut packet_bytes = vec![0; len as usize];

    stdout.read_exact(&mut packet_bytes)?;

    Ok(Some(bincode::decode_from_slice(&packet_bytes, bincode::config::standard())?.0))
}
