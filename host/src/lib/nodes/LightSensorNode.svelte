<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useHandleConnections,
        useNodesData,
    } from "@xyflow/svelte";
    import { AdiPortHandle, DataHandle } from "~/lib/handles";
    import {
        Field,
        NumberInput,
        NodeBase,
        Slider,
    } from "~/lib/components";
    import { LightSensor } from "~/lib/icons";

    type NodeData = {};

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    let brightness = 127;

    const brightnessConnections = useHandleConnections({ nodeId: id, type: "target", id: "data_distance"});
    $: distanceData = useNodesData($brightnessConnections[0]?.source);

    $: {
        if ($distanceData) {
            brightness = $distanceData.data.value as number;
        }
    }

    data;
</script>

<NodeBase title="Light Sensor">
    <AdiPortHandle
        slot="handle"
        id="connector"
        type="source"
        parentNode={id}
        position={Position.Left}
    />
    <LightSensor slot="icon" size="16" />

    <Field label="Darkness">
        <DataHandle
            slot="handle"
            id="distance"
            position={Position.Right}
            type="target"
            parentNode={id}
        />
        <NumberInput
                max={255}
                min={0}
                step="1"
                disabled={$brightnessConnections.length > 0}
                bind:value={brightness}
            />
    </Field>
    <Slider bind:value={brightness}  min={0} max={255} step={1} label="Darkness slider" />
</NodeBase>

<style>
    :global(.svelte-flow__node-light_sensor) {
        --ui-hue: var(--adi-hue);
    }
</style>
