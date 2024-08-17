import type { Terminal } from "@xterm/xterm";
import type Session from "~/lib/session";
import type { Edge, Node } from "@xyflow/svelte";

import { writable, type Writable } from "svelte/store";

export const session: Writable<Session | null> = writable(null);
export const terminal: Writable<Terminal | null> = writable(null);
export const display: Writable<CanvasRenderingContext2D | null> =
    writable(null);

export const dndType: Writable<string | null> = writable(null);
export const nodes = writable<Node[]>([
    {
        id: "brain",
        type: "brain",
        data: {},
        position: { x: 0, y: 0 },
        deletable: false,
    },
    {
        id: "battery",
        type: "battery",
        data: {
            capacity: 0,
            temperature: 0,
            current: 0,
            voltage: 0,
        },
        position: { x: 590, y: 225 },
        deletable: false,
    },
    {
        id: "onboard_adi",
        type: "adi",
        data: {
            onboard: true,
        },
        position: { x: -100, y: 0 },
        deletable: false,
    },
]);
export const edges = writable<Edge<any>[]>([
    {
        id: "battery_connection",
        type: "default",
        source: "battery",
        target: "brain",
        sourceHandle: "connector",
        targetHandle: "battery_port",
        deletable: false,
    },
    {
        id: "onboard_adi_connection",
        type: "default",
        source: "onboard_adi",
        target: "brain",
        sourceHandle: "connector",
        targetHandle: "onboard_adi_port",
        deletable: false,
    },
]);
