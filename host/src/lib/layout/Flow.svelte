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

    import { dndType } from "~/lib/stores";

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
