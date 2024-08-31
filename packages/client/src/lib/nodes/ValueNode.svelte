<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Position,
        useSvelteFlow,
    } from "@xyflow/svelte";
    import {
        Field,
        NumberInput,
        NodeBase,
        Tabs,
        TabPanel,
        Select,
    } from "~/lib/components";
    import { DataHandle } from "~/lib/handles";
    import { Hash } from "svelte-feathers";
    import { writable, type Writable } from "svelte/store";

    type NodeData = {
        value: Writable<number>;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData = { value: writable(0) };
    export let id: $$Props["id"];

    const { value } = data;

    let currentTab = "value";

    let constants = [
        { value: "pi", label: "π" },
        { value: "tao", label: "τ" },
        { value: "e", label: "e" },
    ];
    let constant = constants[0].value;

    $: {
        if (currentTab === "constant") {
            switch (constant) {
                case "pi":
                    $value = Math.PI;
                    break;
                case "tao":
                    $value = 2 * Math.PI;
                    break;
                case "e":
                    $value = Math.E;
                    break;
            }
        }
    }

    data;
</script>

<NodeBase title="Value">
    <Hash slot="icon" size="16" />
    <DataHandle
        slot="handle"
        id="output"
        position={Position.Left}
        type="source"
        parentNode={id}
    />
    <Tabs bind:selected={currentTab}>
        <TabPanel label="Value" id="value">
            <Field label="Value"><NumberInput bind:value={$value} /></Field>
        </TabPanel>
        <TabPanel label="Constant" id="constant">
            <Field label="Constant"
                ><Select bind:selected={constant}>
                    {#each constants as { value, label }}
                        <option {value}>{label}</option>
                    {/each}
                </Select></Field
            >
        </TabPanel>
    </Tabs>
</NodeBase>

<style>
    :global(.svelte-flow__node-value) {
        --ui-hue: var(--data-hue);
    }
</style>
