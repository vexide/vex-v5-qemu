#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use bincode::{Decode, Encode};
use code_signature::CodeSignature;

pub mod code_signature;

/// A message sent from the guest to the host.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Encode, Decode)]
pub enum HostBoundPacket {
    Handshake,
    UserSerial(Vec<u8>),
    KernelSerial(Vec<u8>),
    CodeSignature(CodeSignature),
    ExitRequest(i32),
}

/// A message sent from the host to the guest.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Encode, Decode)]
pub enum GuestBoundPacket {
    Handshake,
    Serial {
        channel: u32,
        data: Vec<u8>,
    },
}
