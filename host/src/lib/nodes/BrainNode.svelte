<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";
    import { path } from "@tauri-apps/api";
    import { session } from "~/lib/stores";
    import Display from "~/lib/Display.svelte";
    import { derived, type Readable } from "svelte/store";

    type $$Props = NodeProps<Node<NodeData>>;

    type NodeData = {};

    export let data: NodeData;
    data;

    const programName: Readable<string> = derived(session, ($session, set) => {
        if ($session) {
            path.basename($session.binary || "", ".bin").then((name) =>
                set(name),
            );
        }

        return () => {
            set = () => {};
        };
    });
</script>

<div class="ports ports-top">
    {#each { length: 10 } as _, n}
        <Handle id="smart_port_{n + 1}" type="target" position={Position.Top} />
    {/each}
</div>

<Display programName={$programName} />

<div class="ports ports-bottom">
    {#each { length: 10 } as _, n}
        <Handle
            id="smart_port_{n + 11}"
            type="target"
            position={Position.Top}
        />
    {/each}
</div>

<Handle
    id="smart_port_21"
    style="top: 33.33%;"
    type="target"
    position={Position.Right}
/>
<Handle
    id="battery_port"
    style="top: 66.66%;"
    type="target"
    isConnectable={false}
    position={Position.Right}
/>
<Handle
    id="onboard_adi_port"
    type="target"
    isConnectable={false}
    position={Position.Left}
/>

<style>
    :global(.svelte-flow__node-brain .display) {
        border-radius: 4px;
    }

    .ports {
        z-index: 1;
        position: absolute;
        left: 50%;
        width: 100%;
        display: flex;
        justify-content: space-evenly;
    }

    .ports-top {
        top: 0;
        transform: translateX(-50%) translateY(-50%);
    }

    .ports-bottom {
        bottom: 0;
        transform: translateX(-50%) translateY(50%);
    }

    .ports > :global(.svelte-flow__handle) {
        position: relative;
        transform: none;
        top: 0;
        left: 0;
    }
</style>
