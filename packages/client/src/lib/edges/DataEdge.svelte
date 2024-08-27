<script lang="ts">
    import {
        getBezierPath,
        BaseEdge,
        type EdgeProps,
        EdgeLabelRenderer,
        useEdges,
    } from "@xyflow/svelte";

    type $$Props = EdgeProps;

    export let selected: $$Props["selected"] = false;
    export let sourceX: $$Props["sourceX"];
    export let sourceY: $$Props["sourceY"];
    export let sourcePosition: $$Props["sourcePosition"];
    export let targetX: $$Props["targetX"];
    export let targetY: $$Props["targetY"];
    export let targetPosition: $$Props["targetPosition"];
    export let markerEnd: $$Props["markerEnd"] = undefined;
    export let style: $$Props["style"] = undefined;

    $: [edgePath] = getBezierPath({
        sourceX,
        sourceY,
        sourcePosition,
        targetX,
        targetY,
        targetPosition,
    });
</script>

<BaseEdge path={edgePath} {markerEnd} {style} class="data-edge" />

<style>
    :global(.svelte-flow__edge:has(.data-edge)) {
        --ui-hue: var(--data-hue);
    }
</style>
