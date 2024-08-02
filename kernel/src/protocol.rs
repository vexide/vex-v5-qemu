use snafu::Snafu;
use bincode::error::{DecodeError, EncodeError};
use semihosting::io::{stdin, stdout, ErrorKind, Read, Write};
use vex_v5_qemu_protocol::{GuestBoundPacket, HostBoundPacket};

use alloc::{vec::Vec, vec};

#[derive(Debug, Snafu)]
pub enum ProtocolError {
    Io { inner: semihosting::io::Error },
    ReadAhead,
    Decode { inner: DecodeError },
    Encode { inner: EncodeError },
}

pub fn send_packet(packet: HostBoundPacket) -> Result<(), ProtocolError> {
    let mut stdout = stdout().map_err(|err| ProtocolError::Io { inner: err })?;
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard())
        .map_err(|err| ProtocolError::Encode { inner: err })?;

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    stdout
        .write_all(&bytes)
        .map_err(|err| ProtocolError::Io { inner: err })?;

    Ok(())
}

pub fn recv_packet() -> Result<Option<GuestBoundPacket>, ProtocolError> {
    let mut stdin = stdin().map_err(|err| ProtocolError::Io { inner: err })?;

    // Read packet length
    let len = {
        let mut buf = [0; 4];

        if let Err(err) = stdin.read(&mut buf) {
            if err.kind() == ErrorKind::UnexpectedEof {
                return Ok(None);
            } else {
                return Err(ProtocolError::Io {
                    inner: err,
                });
            }
        }

        u32::from_le_bytes(buf)
    };

    // Read packet payload
    let mut packet_bytes = vec![0; len as usize];

    stdin
        .read_exact(&mut packet_bytes)
        .map_err(|err| ProtocolError::Io { inner: err })?;

    Ok(Some(bincode::decode_from_slice(&packet_bytes, bincode::config::standard())
        .map_err(|err| ProtocolError::Decode { inner: err })?.0))
}
