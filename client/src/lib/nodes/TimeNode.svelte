<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import {
        Button,
        Field,
        NodeBase,
        NumberDisplay,
        NumberInput,
        Switch,
    } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";
    import { Clock, Hash } from "svelte-feathers";
    import { onDestroy, onMount } from "svelte";

    type NodeData = {
        value: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    const { updateNodeData } = useSvelteFlow();

    let start: number;
    let now: number;

    const { setInterval, clearInterval } = window;

    let updateRate = 60;

    let timeInterval: number | undefined;
    let updateInterval: number | undefined;

    onMount(() => {
        start = performance.now() / 1000;

        timeInterval = setInterval(() => {
            now = performance.now() / 1000;
        }, 1);
        updateInterval = setInterval(() => {
            updateNodeData(id, { value: now - start });
        }, 1000 / updateRate);
    });

    function updateRateChanged(newRate: number) {
        if (updateInterval && newRate) {
            clearInterval(updateInterval);
            updateRate = newRate;
            updateInterval = setInterval(() => {
                updateNodeData(id, { value: now - start });
            }, 1000 / newRate);
        }
    }
    $: updateRateChanged(updateRate);

    onDestroy(() => {
        clearInterval(timeInterval);
        clearInterval(updateInterval);
    });

    let advanced = false;

    data;
</script>

<NodeBase title="Time">
    <Clock slot="icon" size="16" />
    <DataHandle
        slot="handle"
        id="output"
        position={Position.Left}
        type="source"
        parentNode={id}
    />
    <Field label="Seconds"><NumberDisplay value={now - start} /></Field>
    <Field label="Advanced"><Switch bind:flipped={advanced}></Switch></Field>
    {#if advanced}
        <Field label="Update Rate (Hz)"><NumberInput bind:value={updateRate} min={1} max={1000}/></Field>
    {/if}
</NodeBase>

<style>
    :global(.svelte-flow__node-time) {
        --ui-hue: var(--data-hue);
    }
</style>
