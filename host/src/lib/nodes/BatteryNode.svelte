<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";
    import { Battery } from "svelte-feathers";
    import { Field, NumberInput, NodeBase } from "~/lib/components";

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
        <NumberInput bind:value={data.capacity} />
    </Field>
    <Field label="Temperature">
        <NumberInput bind:value={data.temperature} />
    </Field>
    <Field label="Current">
        <NumberInput bind:value={data.current} />
    </Field>
    <Field label="Voltage">
        <NumberInput bind:value={data.voltage} />
    </Field>
</NodeBase>
