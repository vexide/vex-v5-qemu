use tauri_plugin_shell::process::CommandChild;
use vex_v5_qemu_protocol::KernelBoundPacket;

/// Sends a [`KernelBoundPacket`] to the kernel process's stdin stream.
pub fn send_packet(child: &mut CommandChild, packet: KernelBoundPacket) {
    let encoded = bincode::encode_to_vec(packet, bincode::config::standard()).unwrap();

    let mut bytes = Vec::new();

    bytes.extend((encoded.len() as u32).to_le_bytes());
    bytes.extend(encoded);

    child.write(&bytes).unwrap();
}
