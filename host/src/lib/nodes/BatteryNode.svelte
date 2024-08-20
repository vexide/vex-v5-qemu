<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";
    import { Battery } from "svelte-feathers";
    import { Field, NumberInput, NodeBase } from "~/lib/components";
    import Slider from "../components/Slider.svelte";
    import Tabs from "../components/Tabs.svelte";
    import TabPanel from "../components/TabPanel.svelte";

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

<NodeBase class="battery-node" title="Battery">
    <Handle
        slot="handle"
        id="connector"
        type="source"
        isConnectable={false}
        position={Position.Left}
    />
    <Battery slot="icon" size="16" />

    <Tabs selected="manual">
        <TabPanel id="auto" label="Auto">
            TODO
        </TabPanel>
        <TabPanel id="manual" label="Manual">
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
        </TabPanel>
    </Tabs>
</NodeBase>

<style>
    :global(.battery-node) {
        min-width: 200px;
    }
</style>
