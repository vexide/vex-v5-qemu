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
        id: `smart_port_${id}`,
        type,
    });
</script>

<Handle
    id="smart_port_{id}"
    class="smart-port-handle"
    isConnectable={$connections.length === 0}
    isValidConnection={c => {
        const isSourceSmartport = c.sourceHandle?.startsWith("smart_port") ?? false;
        const isTargetSmartport = c.targetHandle?.startsWith("smart_port") ?? false;
        return isSourceSmartport && isTargetSmartport;
    }}
    {style}
    {position}
    {type}
/>
