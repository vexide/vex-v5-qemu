use alloc::{vec, vec::Vec};
use bincode::error::{DecodeError, EncodeError};
use embedded_io::{Read, ReadExactError, Write};
use snafu::Snafu;
use vex_v5_qemu_protocol::{GuestBoundPacket, HostBoundPacket};

use crate::peripherals::UART1;
use crate::protocol;

#[derive(Debug, Snafu)]
pub enum ProtocolError {
    Decode { inner: DecodeError },
    Encode { inner: EncodeError },
}

pub fn send_packet(packet: HostBoundPacket) -> Result<(), ProtocolError> {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard())
        .map_err(|err| ProtocolError::Encode { inner: err })?;

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    UART1.lock().write_all(&bytes).unwrap();

    Ok(())
}

pub fn recv_packet() -> Result<Option<GuestBoundPacket>, ProtocolError> {
    let mut uart = UART1.lock();

    // Read packet length
    let len = {
        let mut buf = [0; 4];

        if let Err(err) = uart.read_exact(&mut buf) {
            if err == ReadExactError::UnexpectedEof {
                return Ok(None);
            } else {
                unreachable!()
            }
        }

        u32::from_le_bytes(buf)
    };

    // Read packet payload
    let mut packet_bytes = vec![0; len as usize];

    uart.read_exact(&mut packet_bytes)
        .expect("packet was not of expected length");

    Ok(Some(
        bincode::decode_from_slice(&packet_bytes, bincode::config::standard())
            .map_err(|err| ProtocolError::Decode { inner: err })?
            .0,
    ))
}

pub fn exit(code: i32) -> ! {
    protocol::send_packet(HostBoundPacket::ExitRequest(code)).unwrap();

    loop {
        core::hint::spin_loop();
    }
}
