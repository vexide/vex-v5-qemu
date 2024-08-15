<script lang="ts">
    import { Background, BackgroundVariant, Controls, MiniMap, SvelteFlow, useSvelteFlow, type Edge, type Node, type NodeTypes } from "@xyflow/svelte";
    import { getContext } from "svelte";
    import type { Writable } from "svelte/store";

    const dndType = getContext("dnd") as Writable<string | null>;
    const { screenToFlowPosition } = useSvelteFlow();

    export let nodeTypes: NodeTypes | undefined;
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

        if (!$dndType) {
            return;
        }

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
    {nodes}
    {edges}
    on:dragover={handleFlowDragOver}
    on:drop={handleFlowDrop}
>
    <Background variant={BackgroundVariant.Lines} />
    <Controls />
    <MiniMap />
</SvelteFlow>
