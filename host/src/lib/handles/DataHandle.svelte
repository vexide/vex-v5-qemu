<script lang="ts">
    import { Handle, useHandleConnections, type HandleProps } from "@xyflow/svelte";

    type Props = HandleProps

    export let id: string;
    export let parentNode: string;
    export let type: Props["type"];
    export let position: Props["position"];
    export let style: Props["style"] | undefined = undefined;

    const connections = useHandleConnections({
        nodeId: parentNode,
        id: `data_${id}`,
        type,
    });
</script>

<Handle
    id="data_{id}"
    class="data"
    isConnectable={$connections.length === 0}
    isValidConnection={c => {
        const isSourceData = c.sourceHandle?.startsWith("data") ?? false;
        const isTargetData = c.targetHandle?.startsWith("data") ?? false;
        return isSourceData && isTargetData;
    }}
    {style}
    {position}
    {type}
/>
