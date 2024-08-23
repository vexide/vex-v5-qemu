<script lang="ts">
    import { Handle, useHandleConnections, type HandleProps, useEdges, useSvelteFlow } from "@xyflow/svelte";

    type Props = HandleProps

    export let id: string;
    export let parentNode: string;
    export let type: Props["type"];
    export let position: Props["position"];
    export let style: Props["style"] | undefined = undefined;

    const connections = useHandleConnections({
        nodeId: parentNode,
        id: `adi_port_${id}`,
        type,
    });

    const { updateNodeData } = useSvelteFlow();

    const edges = useEdges();
    $: {
        if (type === "source") {
            let connection_edges = $connections
                .map((c) => $edges.find((e) => e.id === c.edgeId))
                .filter((e) => e);
            connection_edges.map((e) => (e!.type = "adi"));
            updateNodeData(parentNode, {});
        }
    }
</script>

<Handle
    id="adi_port_{id}"
    class="adi-port-handle"
    isConnectable={$connections.length === 0}
    isValidConnection={c => {
        const isSourceSmartport = c.sourceHandle?.startsWith("adi_port") ?? false;
        const isTargetSmartport = c.targetHandle?.startsWith("adi_port") ?? false;
        return isSourceSmartport && isTargetSmartport;
    }}
    {style}
    {position}
    {type}
/>

<style>
    :global(.svelte-flow__handle.adi-port-handle) {
        border-color: var(--adi-faded);
    }
</style>
