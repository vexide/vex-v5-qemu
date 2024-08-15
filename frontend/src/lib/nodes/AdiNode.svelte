<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";

    type NodeData = {
        onboard: boolean;
    };

    type $$Props = NodeProps<Node<NodeData>>;

    export let data: NodeData;
</script>

<div class="adi-inner" class:onboard={data.onboard}>
    ADI
    <Handle
        id="connector"
        type="source"
        isConnectable={!data.onboard}
        position={data.onboard ? Position.Right : Position.Bottom}
    />
    <div class="ports">
        {#each ["a", "b", "c", "d", "e", "f", "g", "h"] as port}
            <Handle
                id="adi_{port}"
                type="target"
                position={data.onboard ? Position.Left : Position.Top}
            />
        {/each}
    </div>
</div>

<style>
    .adi-inner {
        width: 200px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .adi-inner.onboard {
        height: 200px;
        width: auto;
    }

    .ports {
        position: absolute;
        left: 50%;
        width: 100%;
        display: flex;
        justify-content: space-evenly;
        top: 0;
        transform: translateX(-50%) translateY(-50%);
    }

    .adi-inner.onboard .ports {
        top: 50%;
        left: 0;
        flex-direction: column;
        height: 100%;
        width: auto;
        transform: translateX(-50%) translateY(-50%);
    }

    .ports > :global(.svelte-flow__handle) {
        position: relative;
        transform: none;
        top: 0;
        left: 0;
    }
</style>
