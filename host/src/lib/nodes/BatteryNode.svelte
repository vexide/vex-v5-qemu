<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";
    import { Battery } from "svelte-feathers";
    import { Field, NumberInput } from "~/lib/components";
    import NodeBase from "../components/NodeBase.svelte";

    type NodeData = {
        capacity: number;
        temperature: number;
        current: number;
        voltage: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    data;
</script>

<NodeBase title="Battery">
    <Handle
        slot="handle"
        id="connector"
        type="source"
        isConnectable={false}
        position={Position.Left}
    />
    <Battery slot="icon" size="16" />

    <Field label="Capacity">
        <NumberInput label="Capacity" bind:value={data.capacity} />
    </Field>
    <Field label="Temperature">
        <NumberInput label="Temperature" bind:value={data.temperature} />
    </Field>
    <Field label="Current">
        <NumberInput label="Current" bind:value={data.current} />
    </Field>
    <Field label="Voltage">
        <NumberInput label="Voltage" bind:value={data.voltage} />
    </Field>
</NodeBase>
