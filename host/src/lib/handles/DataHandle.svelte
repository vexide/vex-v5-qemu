<script lang="ts">
    import {
        Handle,
        useHandleConnections,
        type HandleProps,
        useEdges,
        useSvelteFlow,
    } from "@xyflow/svelte";

    type Props = HandleProps;

    export let id: string;
    export let parentNode: string;
    export let type: Props["type"];
    export let position: Props["position"];
    export let style: Props["style"] | undefined = undefined;

    const { updateNodeData } = useSvelteFlow();

    const connections = useHandleConnections({
        nodeId: parentNode,
        id: `data_${id}`,
        type,
    });
    const edges = useEdges();
    $: {
        if (type === "source") {
            let connection_edges = $connections
                .map((c) => $edges.find((e) => e.id === c.edgeId))
                .filter((e) => e);
            connection_edges.map((e) => (e!.type = "data"));
            updateNodeData(parentNode, {});
        }
    }
</script>

<Handle
    id="data_{id}"
    class="data-handle"
    isConnectable={type === "source" || $connections.length === 0}
    isValidConnection={c => {
        const isSourceData = c.sourceHandle?.startsWith("data") ?? false;
        const isTargetData = c.targetHandle?.startsWith("data") ?? false;
        return isSourceData && isTargetData;
    }}
    {style}
    {position}
    {type}
/>

<style>
    :global(.svelte-flow__handle.data-handle) {
        --ui-hue: var(--data-hue);
    }
</style>
