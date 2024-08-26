<script lang="ts">
    import { useSvelteFlow, type Node, type NodeTypes } from "@xyflow/svelte";
    import type { DragData } from "../layout/Sidebar.svelte";
    import { nodes } from "../stores.js";
    import NodeBase from "./NodeBase.svelte";
    import type { SvelteComponent } from "svelte";

    const { screenToFlowPosition, viewport } = useSvelteFlow();

    export let dragNode: DragData | null = null;
    export let nodeTypes: NodeTypes;

    function handleNodeDrop() {
        if (!dragNode) return;

        if (dragNode.valid) {
            const position = screenToFlowPosition({
                x: dragNode.x,
                y: dragNode.y,
            });

            const headerOffset = -18;
            position.y += headerOffset;

            const newNode = {
                id: `${crypto.randomUUID()}`,
                type: dragNode.nodeType,
                position,
                data: { label: `${dragNode.nodeType} node` },
                origin: [0.5, 0.0],
            } satisfies Node;

            $nodes.push(newNode);
            $nodes = $nodes;
        }

        dragNode = null;
    }
</script>

<svelte:window
    on:mousemove={(e) => {
        if (dragNode) {
            dragNode.x = e.clientX;
            dragNode.y = e.clientY;
        }
    }}
    on:mouseup={handleNodeDrop}
/>

<div class="drag-overlay">
    {#if dragNode}
        <div
            class="drag-item svelte-flow__node"
            data-valid={dragNode.valid}
            style:left="{dragNode.x}px"
            style:top="{dragNode.y}px"
            style:--flow-zoom={$viewport.zoom}
        >
            <svelte:component
                this={nodeTypes[dragNode.nodeType] ?? NodeBase}
                title="{dragNode.nodeType} node"
                data={{}}
            />
        </div>
    {/if}
</div>

<style>
    .drag-overlay {
        position: absolute;
        z-index: 1000;
        pointer-events: none;
        inset: 0;
        overflow: hidden;
    }
    .drag-item {
        position: absolute;
        z-index: 1000;
        transform: scale(var(--flow-zoom, 1)) translate(-50%, calc(-36px / 2));
        pointer-events: none;
    }

    .drag-item[data-valid="false"] {
        opacity: 0.5;
        filter: grayscale(0.75);
    }
</style>
