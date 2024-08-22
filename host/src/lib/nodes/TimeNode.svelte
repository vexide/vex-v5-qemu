<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import {
        Field,
        NodeBase,
        NumberDisplay,
    } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";
    import { Clock, Hash } from "svelte-feathers";
    import { onMount } from "svelte";

    type NodeData = {
        value: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    const { updateNodeData } = useSvelteFlow();

    let start: number;
    let now: number;

    onMount(() => {
        start = performance.now() / 1000;

        setInterval(() => {
            now = performance.now() / 1000;
            updateNodeData(id, { value: now - start })
        }, 1);
    });


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
</NodeBase>
