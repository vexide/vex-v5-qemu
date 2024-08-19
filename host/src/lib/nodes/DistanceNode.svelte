<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import { drag } from "~/lib/actions";
    import { SmartPortHandle } from "~/lib/handles"
    import { Field, NumberInput, NodeBase, Checkbox, Divider } from "~/lib/components";
    import { DistanceSensor } from "~/lib/icons";

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

<NodeBase title="Distance Sensor">
    <SmartPortHandle
        slot="handle"
        id="connector"
        type="source"
        parentNode={id}
        position={Position.Left}
    />
    <DistanceSensor slot="icon" size="16" />

    <Field label="Object">
        <Checkbox bind:checked={objectVisible} />
    </Field>
    {#if objectVisible}
        <Divider />
        <Field label="Distance">
            <NumberInput max="2000" min="20" step="50" bind:value={distance} />
        </Field>
        <Field label="Size">
            <NumberInput max="400" min="0" step="10" bind:value={size} />
        </Field>
    {/if}
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
</NodeBase>

<style>
    .distance-visualizer {
       margin-top: 16px;
       display: flex;
       flex-direction: row;
       min-width: 212px;
       width: 100%;
       height: 75px;
       border: 2px solid var(--accent-primary);
       border-radius: 16px;
       align-items: center;
       gap: 8px;
       padding-inline: 8px;
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
