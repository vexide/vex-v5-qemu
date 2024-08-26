<script lang="ts">
    import { useSvelteFlow, type Node, type NodeTypes } from "@xyflow/svelte";
    import type { DragData } from "../layout/Sidebar.svelte";
    import { dndNode, nodes } from "../stores.js";
    import NodeBase from "./NodeBase.svelte";
    import type { SvelteComponent } from "svelte";

    const { screenToFlowPosition } = useSvelteFlow();

    export let dragNode: DragData | null = null;
    export let nodeTypes: NodeTypes;

    function handleNodeDrop() {
        if (!dragNode) return;

        const position = screenToFlowPosition({
            x: dragNode.x,
            y: dragNode.y,
        });

        const newNode = {
            id: `${Math.random()}`,
            type: dragNode.nodeType,
            position,
            data: { label: `${dragNode.nodeType} node` },
            origin: [0.5, 0.0],
        } satisfies Node;

        $nodes.push(newNode);
        $nodes = $nodes;

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

{#if dragNode}
    <div
        class="drag-item"
        data-valid={dragNode.valid}
        style="left: {dragNode.x}px; top: {dragNode.y}px"
    >
        <svelte:component
            this={nodeTypes[dragNode.nodeType] ?? NodeBase}
            title="{dragNode.nodeType} node"
        />
    </div>
{/if}

<style>
    .drag-item {
        position: absolute;
        z-index: 1000;
        transform: translate(-50%, calc(-36px / 2));
        pointer-events: none;
    }

    .drag-item[data-valid="false"] {
        opacity: 0.5;
        filter: grayscale(0.75);
    }
</style>
