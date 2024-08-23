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

    $: style = selected ? "stroke: #98C2EC" : "stroke: #98C2EC7F";

    $: [edgePath] = getBezierPath({
        sourceX,
        sourceY,
        sourcePosition,
        targetX,
        targetY,
        targetPosition,
    });
</script>

<BaseEdge path={edgePath} {markerEnd} {style} />

<style>
</style>
