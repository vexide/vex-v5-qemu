<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import { Field, NumberInput, NodeBase } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";

    type NodeData = {
        value: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    let value = 0;

    const { updateNodeData } = useSvelteFlow();
    $: updateNodeData(id, { value });

    data;
</script>

<NodeBase title="Value">
    <DataHandle
        slot="handle"
        id="output"
        position={Position.Left}
        type="source"
        parentNode={id}
    />
    <Field label="Value">
        <NumberInput bind:value />
    </Field>
</NodeBase>
