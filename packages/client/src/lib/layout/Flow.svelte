<script lang="ts">
    import type { Writable } from "svelte/store";
    import {
        Background,
        BackgroundVariant,
        Controls,
        MiniMap,
        SvelteFlow,
        useSvelteFlow,
        type Edge,
        type Node,
        type NodeTypes,
        type EdgeTypes,
    } from "@xyflow/svelte";

    import { dndType, interpreter } from "~/lib/stores";
    import Interpreter from "~/lib/interpreter";
    import { onDestroy, onMount } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import type { NodeGraphUpdatePayload } from "~/lib/payload";
    import { invoke } from "@tauri-apps/api/core";

    const { screenToFlowPosition } = useSvelteFlow();

    export let nodeTypes: NodeTypes | undefined;
    export let edgeTypes: EdgeTypes | undefined;
    export let nodes: Writable<Node[]>;
    export let edges: Writable<Edge[]>;

    function handleFlowDragOver(event: DragEvent) {
        event.preventDefault();

        if (event.dataTransfer) {
            event.dataTransfer.dropEffect = "move";
        }
    }

    function handleFlowDrop(event: DragEvent) {
        event.preventDefault();

        if (!$dndType) return;

        const position = screenToFlowPosition({
            x: event.clientX,
            y: event.clientY,
        });

        const newNode = {
            id: `${Math.random()}`,
            type: $dndType,
            position,
            data: { label: `${$dndType} node` },
            origin: [0.5, 0.0],
        } satisfies Node;

        $nodes.push(newNode);
        $nodes = $nodes;
    }

    let updateUnlisten: UnlistenFn | undefined;

    onMount(async () => {
        $interpreter = new Interpreter();
        $interpreter.start();
        console.log("interpreter", $interpreter);

        updateUnlisten = await listen<NodeGraphUpdatePayload>("node-graph-update", (event) => {
            console.log("node-graph-update", event);
        })
    });

    onDestroy(() => {
        updateUnlisten?.();
    });

    $: {
        console.log($nodes);
        console.log($edges);
        $interpreter?.update({ brain: { port_1: "distance-node-0" }, nodes: { "distance-node-0": { data: { type: "DistanceSensor", data: { distance: 1000, size: 200 } } } } });
    }
</script>

<SvelteFlow
    {nodeTypes}
    {edgeTypes}
    {nodes}
    {edges}
    fitView
    fitViewOptions={{ maxZoom: 1.0 }}
    on:dragover={handleFlowDragOver}
    on:drop={handleFlowDrop}
>
    <Background variant={BackgroundVariant.Lines} />
    <Controls />
    <MiniMap />
</SvelteFlow>
