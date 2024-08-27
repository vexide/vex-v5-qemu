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

export interface Brain {
    port_1?: string | undefined,
    port_2?: string | undefined,
    port_3?: string | undefined,
    port_4?: string | undefined,
    port_5?: string | undefined,
    port_6?: string | undefined,
    port_7?: string | undefined,
    port_8?: string | undefined,
    port_9?: string | undefined,
    port_10?: string | undefined,
    port_11?: string | undefined,
    port_12?: string | undefined,
    port_13?: string | undefined,
    port_14?: string | undefined,
    port_15?: string | undefined,
    port_16?: string | undefined,
    port_17?: string | undefined,
    port_18?: string | undefined,
    port_19?: string | undefined,
    port_20?: string | undefined,
    port_21?: string | undefined
}

export interface Input {
    source_id: string,
    target_handle_id: string,
    source_handle_id: string,
}

export interface DistanceSensor {
    distance?: number | undefined,
    size?: number | undefined
}

export interface LightSensor {
    darkness?: number | undefined
}

export interface Value {
    value?: number | undefined
}

export interface Math {
    operation: string,
    lhs?: number | undefined
    rhs?: number | undefined
}

export interface Node {
    data: {
        type: "DistanceSensor" | "LightSensor" | "Value" | "Math",
        data: DistanceSensor | LightSensor | Value | Math
    } | "Time"
    inputs?: Input[] | undefined
}

export interface NodeGraph { brain: Brain, nodes: { [key: string]: Node } }

export function updateNodeGraph(nodeGraph: NodeGraph) {
    if ("__TAURI_INTERNALS__" in window) {
        invoke("update_node_graph", { opts: nodeGraph });
    }
}
