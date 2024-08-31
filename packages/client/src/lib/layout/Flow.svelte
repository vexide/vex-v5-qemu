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

    const { screenToFlowPosition } = useSvelteFlow();

    export let nodeTypes: NodeTypes | undefined;
    export let edgeTypes: EdgeTypes | undefined;
    export let nodes: Writable<Node[]>;
    export let edges: Writable<Edge[]>;
</script>

<SvelteFlow
    {nodeTypes}
    {edgeTypes}
    {nodes}
    {edges}
    fitView
    fitViewOptions={{ maxZoom: 1.0 }}
>
    <Background variant={BackgroundVariant.Lines} />
    <Controls />
    <MiniMap />
</SvelteFlow>
