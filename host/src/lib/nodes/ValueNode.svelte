<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import { Field, NumberInput, NodeBase } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";

    type NodeData = {};

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    export let value = 0;

    data;
</script>

<NodeBase title="Value">
    <DataHandle
        slot="handle"
        id="distance"
        position={Position.Left}
        type="source"
        parentNode={id}
    />
    <Field label="Value">
        <NumberInput bind:value />
    </Field>
</NodeBase>
