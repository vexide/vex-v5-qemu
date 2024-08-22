<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
        useNodesData,
        useHandleConnections,
    } from "@xyflow/svelte";
    import {
        Field,
        NumberInput,
        NodeBase,
        RadioGroup,
        Select,
    } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";

    type NodeData = {
        value: number;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
    export let id: $$Props["id"];

    export let lhs = 0;
    export let rhs = 0;

    const operations = [
        {
            value: "add",
            label: "Addition",
        },
        {
            value: "sub",
            label: "Subtraction",
        },
        {
            value: "mul",
            label: "Multiplication",
        },
        {
            value: "div",
            label: "Division",
        },
    ];
    let operation = operations[0].value;

    let result = 0;
    $: {
        switch (operation) {
            case "add":
                result = lhs + rhs;
                break;
            case "sub":
                result = lhs - rhs;
                break;
            case "mul":
                result = lhs * rhs;
                break;
            case "div":
                result = lhs / rhs;
                break;
        }
    }

    const { updateNodeData } = useSvelteFlow();
    $: updateNodeData(id, { value: result });

    const lhsConnections = useHandleConnections({
        nodeId: id,
        type: "target",
        id: "data_lhs",
    });
    const rhsConnections = useHandleConnections({
        nodeId: id,
        type: "target",
        id: "data_rhs",
    });
    $: lhsData = useNodesData($lhsConnections[0]?.source);
    $: rhsData = useNodesData($rhsConnections[0]?.source);

    $: {
        if ($lhsData) {
            lhs = $lhsData.data.value as number;
        }
        if ($rhsData) {
            rhs = $rhsData.data.value as number;
        }
    }

    $: console.log($rhsConnections);

    data;
</script>

<NodeBase title="Math">
    <DataHandle
        slot="handle"
        id="output"
        position={Position.Left}
        type="source"
        parentNode={id}
    />

    <!-- <RadioGroup options={operations} bind:selected={operation}/> -->
    <Select bind:selected={operation}>
        {#each operations as { value, label }}
            <option {value}>{label}</option>
        {/each}
    </Select>

    <Field label="LHS">
        <DataHandle
            slot="handle"
            id="lhs"
            position={Position.Right}
            type="target"
            parentNode={id}
        />
        <NumberInput bind:value={lhs} disabled={$lhsConnections.length > 0} />
    </Field>
    <Field label="RHS">
        <DataHandle
            slot="handle"
            id="rhs"
            position={Position.Right}
            type="target"
            parentNode={id}
        />
        <NumberInput bind:value={rhs} disabled={$rhsConnections.length > 0}/>
    </Field>
    <Field label="Result">
        {result}
    </Field>
</NodeBase>
