use serde::{Deserialize, Serialize};
use tauri_plugin_shell::process::CommandChild;
use vex_v5_qemu_protocol::{
    display::{Color, DrawCommand, ScrollLocation},
    geometry::Rect,
    KernelBoundPacket,
};

/// Sends a [`KernelBoundPacket`] to the kernel process's stdin stream.
pub fn send_packet(child: &mut CommandChild, packet: KernelBoundPacket) {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard()).unwrap();

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    child.write(&bytes).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DisplayDrawPayload {
    pub command: DrawCommand,
    pub color: Color,
    pub clip_region: Rect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DisplayClearPayload {
    pub color: Color,
    pub clip_region: Rect,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DisplayScrollPayload {
    pub location: ScrollLocation,
    pub lines: i32,
    pub background: Color,
    pub clip_region: Rect,
}
