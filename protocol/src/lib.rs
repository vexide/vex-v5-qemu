#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode};
use code_signature::CodeSignature;
use geometry::Rect;
use display::{Color, DrawCommand, DisplayRenderMode, ScrollLocation, TextMeasurement, TextSize};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod code_signature;
pub mod geometry;
pub mod display;

/// A message sent from the guest to the host.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Encode, Decode)]
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
}

/// A message sent from the host to the guest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KernelBoundPacket {
    Handshake,
    UserSerial(Vec<u8>),
    DisplayTextMeasurement(TextMeasurement),
}
