use alloc::{vec, vec::Vec};

use bincode::error::{DecodeError, EncodeError};
use embedded_io::{Read as EIORead, Write as EIOWrite, ReadExactError};
use semihosting::io::{stdout, ErrorKind, Write};
use snafu::Snafu;
use vex_v5_qemu_protocol::{HostBoundPacket, KernelBoundPacket};

use crate::{peripherals::UART1, protocol};

#[derive(Debug, Snafu)]
pub enum ProtocolError {
    Decode { inner: DecodeError },
    Encode { inner: EncodeError },
}

fn semihosting_write_with_retry(bytes: &[u8]) -> Result<(), semihosting::io::Error> {
    match stdout().unwrap().write_all(bytes) {
        Err(err) if err.kind() == ErrorKind::Other && err.raw_os_error() == Some(0) => {
            semihosting_write_with_retry(bytes) // no idea what the fuck is going on here but sure whatever man
        }
        res => res,
    }
}

pub fn send_packet(packet: HostBoundPacket) -> Result<(), ProtocolError> {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard())
        .map_err(|err| ProtocolError::Encode { inner: err })?;

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    semihosting_write_with_retry(&bytes).unwrap();

    Ok(())
}

pub fn send_packet_uart(packet: HostBoundPacket) -> Result<(), ProtocolError> {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard())
        .map_err(|err| ProtocolError::Encode { inner: err })?;

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    _ = EIOWrite::write(&mut *UART1.lock(), &bytes);

    Ok(())
}

pub fn recv_packet() -> Result<Option<KernelBoundPacket>, ProtocolError> {
    let mut uart = UART1.lock();

    // Read packet length
    let len = {
        let mut buf = [0; 4];

        if let Err(err) = EIORead::read_exact(&mut *uart, &mut buf) {
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

    EIORead::read_exact(&mut *uart, &mut packet_bytes).expect("packet was not of expected length");

    Ok(Some(
        bincode::decode_from_slice(&packet_bytes, bincode::config::standard())
            .map_err(|err| ProtocolError::Decode { inner: err })?
            .0,
    ))
}

pub fn exit(code: i32) -> ! {
    _ = protocol::send_packet(HostBoundPacket::ExitRequest(code));

    loop {
        core::hint::spin_loop();
    }
}
