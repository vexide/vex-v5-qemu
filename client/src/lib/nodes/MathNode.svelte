<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
        useNodesData,
        useHandleConnections,
    } from "@xyflow/svelte";
    import { PlusCircle } from "svelte-feathers";
    import {
        Field,
        NumberInput,
        NodeBase,
        Select,
        Tabs,
        TabPanel,
        NumberDisplay,
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

    const binaryOps = [
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
    const unaryOps = [
        {
            value: "neg",
            label: "Negation",
        },
        {
            value: "abs",
            label: "Absolute Value",
        },
        {
            value: "sqrt",
            label: "Square Root",
        },
        {
            value: "sin",
            label: "Sine",
        },
        {
            value: "cos",
            label: "Cosine",
        },
        {
            value: "tan",
            label: "Tangent",
        },
    ];
    let operation = binaryOps[0].value;

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
            case "neg":
                result = -lhs;
                break;
            case "abs":
                result = Math.abs(lhs);
                break;
            case "sqrt":
                result = Math.sqrt(lhs);
                break;
            case "sin":
                result = Math.sin(lhs);
                break;
            case "cos":
                result = Math.cos(lhs);
                break;
            case "tan":
                result = Math.tan(lhs);
                break;
        }
    }

    let currentTab = "binary";

    $: {
        if (currentTab === "binary") {
            operation = binaryOps[0].value;
        } else {
            operation = unaryOps[0].value;
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

    data;
</script>

<NodeBase title="Math">
    <PlusCircle slot="icon" size="16"/>
    <DataHandle
        slot="handle"
        id="output"
        position={Position.Left}
        type="source"
        parentNode={id}
    />

    <!-- <RadioGroup options={operations} bind:selected={operation}/> -->


    <Tabs bind:selected={currentTab}>
        <TabPanel label="Binary" id="binary">
            <Select bind:selected={operation}>
                {#each binaryOps as { value, label }}
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
        </TabPanel>
        <TabPanel label="Unary" id="unary">
            <Select bind:selected={operation}>
                {#each unaryOps as { value, label }}
                    <option {value}>{label}</option>
                {/each}
            </Select>

            <Field label="Input">
                <DataHandle
                    slot="handle"
                    id="lhs"
                    position={Position.Right}
                    type="target"
                    parentNode={id}
                />
                <NumberInput bind:value={lhs} disabled={$lhsConnections.length > 0} />
            </Field>
        </TabPanel>
    </Tabs>

    <Field label="Result">
        <NumberDisplay value={result} decimals={3} />
    </Field>
</NodeBase>

<style>
    :global(.svelte-flow__node-math) {
        --ui-hue: var(--data-hue);
    }
</style>
