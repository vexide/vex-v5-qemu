import { invoke } from "@tauri-apps/api/core";

interface QemuOptions {
    gdb: boolean;
    qemu: string;
    kernel: string;
    binary: string;
    qemu_args: string[];
}

export function spawnQemu(opts: QemuOptions) {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("spawn_qemu", { opts });
    }
}

export function killQemu() {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("kill_qemu");
    }
}
