<script lang="ts">
    import {
        NodeToolbar,
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";

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

Battery
<Handle
    id="connector"
    type="source"
    isConnectable={false}
    position={Position.Left}
/>
