<script lang="ts">
    import {
        NodeToolbar,
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";

    type NodeData = {
        capacity: number;
        temperature: number;
        current: number;
        voltage: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;

    let position = { x: 150 / 2 - 11, y: 150 / 2 - 11 };
    let draggable: HTMLDivElement;
    let draggableContainer: HTMLDivElement;

    function down(e: MouseEvent) {
        draggable.onmousemove = drag;
    }
    function up(e: MouseEvent) {
        draggable.onmousemove = null;
    }

    function drag(e: MouseEvent) {
        position = { x: position.x + e.movementX, y: position.y + e.movementY };
        position = { x: Math.max(0, Math.min(150 - 22, position.x)), y: Math.max(0, Math.min(150 - 22, position.y)) };
        console.log(e.movementX, e.movementY);
    }

    data;
</script>

GPS
<div class="positions">
    <p>x: {position.x}</p>
    <p>y: {position.y}</p>
</div>
<div bind:this={draggableContainer} class="position nodrag">
    <div
        class="draggable"
        style="left: {position.x}px; top: {position.y}px"
        bind:this={draggable}
        on:mouseup={up}
        on:mousedown={down}
    >
        <svg width="24px" height="24"
            ><circle
                cx="12"
                cy="12"
                r="11"
                stroke="currentColor"
                fill="none"
                stroke-width="2"
            /></svg
        >
    </div>
</div>
<Handle id="connector" type="source" position={Position.Bottom} />

<style>
    .positions {
        display: flex;
        gap: 10px;
    }

    .position {
        background-color: var(--background-secondary);
        width: 150px;
        height: 150px;
        border-radius: 16px;
        border: var(--accent-primary) 2px solid;
    }
    .draggable {
        cursor: move;
        position: relative;
    }
    .draggable:hover {
        cursor: grabbing;
        color: var(--foreground-primary);
    }
</style>
