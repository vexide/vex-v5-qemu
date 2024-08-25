<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useHandleConnections,
        useNodesData,
    } from "@xyflow/svelte";
    import { DataHandle, SmartPortHandle } from "~/lib/handles";
    import {
        Field,
        NumberInput,
        NodeBase,
        Checkbox,
        Divider,
    } from "~/lib/components";
    import { DistanceSensor } from "~/lib/icons";
    import Slider from "../components/Slider.svelte";

    type NodeData = {};

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    let distance = 1000;
    let size = 200;

    let objectVisible = true;

    const distanceConnections = useHandleConnections({ nodeId: id, type: "target", id: "data_distance"});
    const sizeConnections = useHandleConnections({ nodeId: id, type: "target", id: "data_size"});
    $: distanceData = useNodesData($distanceConnections[0]?.source);
    $: sizeData = useNodesData($sizeConnections[0]?.source);

    $: {
        if ($distanceData) {
            distance = $distanceData.data.value as number;
        }
        if ($sizeData) {
            size = $sizeData.data.value as number;
        }
    }

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

    <Field label="Object Detected">
        <Checkbox bind:checked={objectVisible} />
    </Field>
    <Divider />
    <Field label="Distance">
        <DataHandle
            slot="handle"
            id="distance"
            position={Position.Right}
            type="target"
            parentNode={id}
        />
        {#if objectVisible}<NumberInput
                max={2000}
                min={20}
                step="10"
                disabled={!objectVisible && $distanceConnections.length > 0}
                bind:value={distance}
            />{:else}<NumberInput disabled="true" value="9999" />{/if}
    </Field>
    {#if objectVisible}
        <Slider bind:value={distance} disabled={!objectVisible} min={20} max={2000} step={10} label="Distance slider" />
    {/if}
    <Field label="Size">
        <DataHandle
            slot="handle"
            id="size"
            position={Position.Right}
            type="target"
            parentNode={id}
        />
        {#if objectVisible}<NumberInput
                max={400}
                min={0}
                step="10"
                disabled={!objectVisible && $sizeConnections.length > 0}
                bind:value={size}
            />{:else}<NumberInput disabled="true" value="-1" />{/if}
    </Field>
</NodeBase>
