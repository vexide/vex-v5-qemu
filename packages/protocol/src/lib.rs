#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::{num::NonZeroU32, option::Option};

use battery::BatteryData;
use code_signature::CodeSignature;
use controller::{ControllerData, ControllerId};
use display::{Color, DrawCommand, ScrollLocation};
use distance_sensor::DistanceSensorData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use kurbo::{Rect, Circle, Line};

pub mod battery;
pub mod code_signature;
pub mod controller;
pub mod display;
pub mod distance_sensor;
pub mod motor;

/// A message sent from the guest to the host.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HostBoundPacket {
    UsbSerial(Vec<u8>),
    KernelSerial(Vec<u8>),
    CodeSignature(CodeSignature),
    ExitRequest(i32),
    DisplayCommand { command: DisplayCommand },
    SmartPortCommand { port: u8, command: SmartPortCommand },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DisplayCommand {
    Draw {
        command: DrawCommand,
        color: Color,
        clip_region: kurbo::Rect,
    },
    Scroll {
        location: ScrollLocation,
        lines: i32,
        background: Color,
        clip_region: kurbo::Rect,
    },
    Erase {
        color: Color,
        clip_region: kurbo::Rect,
    },
    DisableDoubleBuffering,
    Render,
}

/// A message sent from the host to the guest.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KernelBoundPacket {
    UsbSerial(Vec<u8>),
    LinkAddress(Option<NonZeroU32>),
    SmartPortUpdate {
        port_index: u8,
        data: SmartPortData,
        timestamp: u32,
    },
    ControllerUpdate {
        id: ControllerId,
        data: ControllerData,
        timestamp: u32,
    },
    BatteryUpdate {
        data: BatteryData,
        timestamp: u32,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SmartPortData {
    DistanceSensor(DistanceSensorData),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SmartPortCommand {}
