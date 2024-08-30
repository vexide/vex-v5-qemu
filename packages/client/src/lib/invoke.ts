import { invoke } from "@tauri-apps/api/core";
import type { NodeGraph } from "~/lib/nodeGraph";

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

export function startNodeGraphInterpreter() {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("start_node_graph_interpreter", {});
    }
}
export function stopNodeGraphInterpreter() {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("stop_node_graph_interpreter", {});
    }
}

export function updateNodeGraph(nodeGraph: NodeGraph) {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("update_node_graph", { opts: nodeGraph });
    }
}
