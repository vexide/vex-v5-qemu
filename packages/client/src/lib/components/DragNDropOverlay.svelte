<script lang="ts">
    import { useSvelteFlow, type Node, type NodeTypes } from "@xyflow/svelte";
    import type { DragData } from "../layout/Sidebar.svelte";
    import { nodes } from "../stores.js";
    import NodeBase from "./NodeBase.svelte";
    import { writable } from "svelte/store";

    const { screenToFlowPosition, viewport } = useSvelteFlow();

    export let dragNode: DragData | null = null;
    export let nodeTypes: NodeTypes;

    function defaultNodeData(type: string): Record<string, unknown> {
        console.log(type);
        switch (type) {
            case "distance":
                return { distance: writable(1000), size: writable(200) };
            case "light_sensor":
                return { darkness: 0 };
            case "value":
                return { value: writable(0) };
            case "adi":
                return { onboard: false };
            default:
                return {
                    label: `${type} node`,
                };
        }
    }

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
                data: defaultNodeData(dragNode.nodeType),
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
            class="drag-item svelte-flow__node svelte-flow__node-{dragNode.nodeType}"
            data-valid={dragNode.valid}
            style:left="{dragNode.x}px"
            style:top="{dragNode.y}px"
            style:--flow-zoom={$viewport.zoom}
        >
            <svelte:component
                this={nodeTypes[dragNode.nodeType] ?? NodeBase}
                title="{dragNode.nodeType} node"
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
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.35);
    }

    .drag-item[data-valid="false"] {
        opacity: 0.9;
        filter: grayscale(0.75);
    }
</style>
