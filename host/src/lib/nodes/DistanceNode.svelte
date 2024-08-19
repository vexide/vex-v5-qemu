<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import { drag } from "~/lib/actions";
    import { SmartPortHandle } from "~/lib/handles";

    const { screenToFlowPosition } = useSvelteFlow();

    type NodeData = {};

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    let distance = 1000;
    let size = 200;

    let visualizer: HTMLDivElement;

    function moveObject(e: PointerEvent) {
        let flowCoords = screenToFlowPosition({ x: e.clientX, y: 0 });
        let boundingRect = visualizer.getBoundingClientRect();
        let boundingCoords = screenToFlowPosition({
            x: boundingRect.left,
            y: 0,
        });
        let relativeX = flowCoords.x - boundingCoords.x - 50;
        distance = Math.round(
            Math.max(20, Math.min(2000, (relativeX * 2000) / 100)),
        );
    }

    let objectVisible = true;

    data;
</script>

Distance
<div class="input nodrag">
    <label for="object-detected">Object</label>
    <input type="checkbox" id="object-detected" bind:checked={objectVisible} />
</div>
<br style="display: {objectVisible ? 'block' : 'none'};" />
<div style="width: 100%; display: {objectVisible ? 'block' : 'none'};">
    <div class="input nodrag">
        <label for="distance">Distance</label>
        <input
            class="nodrag"
            type="number"
            id="distance"
            min="20"
            max="2000"
            step="50"
            bind:value={distance}
        />
    </div>
    <div class="input nodrag">
        <label for="size">Size</label>
        <input
            class="nodrag"
            type="number"
            id="size"
            min="10"
            max="400"
            step="10"
            bind:value={size}
        />
    </div>
</div>
<div class="distance-visualizer nodrag" bind:this={visualizer}>
    <svg
        width="25"
        height="25"
        viewBox="0 0 9 10"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
    >
        <path
            d="M6.74028 4.15077L2.77802 1.68718C2.11189 1.27301 1.25 1.75203 1.25 2.53642L1.25 7.46359C1.25 8.24797 2.11189 8.72699 2.77802 8.31282L6.74028 5.84923C7.36959 5.45795 7.36959 4.54205 6.74028 4.15077Z"
            stroke="currentColor"
            stroke-width="2"
        />
    </svg>
    <div
        class="distance {objectVisible ? 'distance-object' : 'distance-no-object'}"
        style="width: {objectVisible ? (distance * 100) / 2000 : 125}px;"
    />
    {#if objectVisible}
        <div
            class="object"
            style="width: {(size * 50) / 400}px; height: {(size * 50) / 400}px;"
            use:drag={(event) => {
                if (!visualizer || !objectVisible) return;
                moveObject(event);
            }}
        />
    {:else}
        <svg
            width="25"
            height="25"
            viewBox="0 0 10 10"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="no-object"
        >
            <path
                d="M1 1L9 9"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
            />
            <path
                d="M9 1L1 9"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
            />
        </svg>
    {/if}
</div>

<SmartPortHandle
    id="connector"
    parentNode={id}
    type="source"
    position={Position.Bottom}
/>

<style>
    .input {
        display: flex;
        width: 100%;
        justify-content: space-between;
    }

    .distance-visualizer {
        margin-top: 16px;
        display: flex;
        flex-direction: row;
        min-width: 200px;
        width: 100%;
        height: 75px;
        background-color: var(--background-secondary);
        border-radius: 16px;
        align-items: center;
        padding-inline: 8px;
        gap: 4px;
    }
    .distance {
        border: solid 4px;
        border-radius: 4px;
    }
    .distance-object {
        max-width: 100px;
        border-color: var(--accent-primary);
        border-style: solid;
    }
    .distance-no-object {
        border-color: var(--foreground-tertiary);
        border-style: dashed;
        width: 100%;
    }

    .object {
        border: solid 5px var(--foreground-secondary);
        border-radius: 50%;
        cursor: move;
        max-width: 50px;
        max-height: 50px;
    }
    .object:hover {
        border-color: var(--foreground-primary);
    }
    .no-object {
        color: var(--foreground-tertiary);
    }
</style>
