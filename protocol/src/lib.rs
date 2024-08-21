#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};
use code_signature::CodeSignature;
use geometry::Rect;
use display::{Color, DrawCommand, DisplayRenderMode, ScrollLocation, TextMeasurement, TextSize};

use motor::{MotorFaults, MotorFlags};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod code_signature;
pub mod geometry;
pub mod display;
pub mod motor;

/// A message sent from the guest to the host.
#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostBoundPacket {
    Handshake,
    UserSerial(Vec<u8>),
    KernelSerial(Vec<u8>),
    CodeSignature(CodeSignature),
    ExitRequest(i32),
    DisplayDraw {
        command: DrawCommand,
        color: Color,
        clip_region: Rect,
    },
    DisplayScroll {
        location: ScrollLocation,
        lines: i32,
        background: Color,
        clip_region: Rect,
    },
    DisplayErase {
        color: Color,
        clip_region: Rect,
    },
    DisplayMeasureText {
        data: String,
        size: TextSize,
    },
    DisplayRenderMode(DisplayRenderMode),
    DisplayRender,
    DeviceCommand {
        port: u8,
        command: DeviceCommand,
    },
}

/// A message sent from the host to the guest.
#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KernelBoundPacket {
    Handshake,
    UserSerial(Vec<u8>),
    DisplayTextMeasurement(TextMeasurement),
    DeviceUpdate {
        port: u8,
        data: DeviceData,
        timestamp: u32,
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceData {
    Motor {
        velocity: f64,
        position: i32,
        power: f64,
        torque: f64,
        efficiency: f64,
        temperature: f64,
        flags: MotorFlags,
        faults: MotorFaults,
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceCommand {}

#[macro_export]
macro_rules! impl_bincode_bitflags {
    ($flags:ty) => {
        impl bincode::enc::Encode for $flags {
            fn encode<E: ::bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.bits(), encoder)?;
                Ok(())
            }
        }

        impl bincode::de::Decode for $flags {
            fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
                Ok(Self::from_bits_retain(bincode::Decode::decode(decoder)?))
            }
        }

        impl<'de> bincode::BorrowDecode<'de> for $flags {
            fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
                Ok(Self::from_bits_retain(bincode::BorrowDecode::borrow_decode(
                    decoder,
                )?))
            }
        }
    }
}
