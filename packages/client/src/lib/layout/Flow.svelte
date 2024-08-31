<script lang="ts">
    import type { Writable } from "svelte/store";
    import {
        Background,
        BackgroundVariant,
        Controls,
        MiniMap,
        SvelteFlow,
        useSvelteFlow,
        type Edge,
        type Node,
        type NodeTypes,
        type EdgeTypes,
    } from "@xyflow/svelte";

    import { interpreter } from "~/lib/stores";
    import { svelteFlowToNodeGraph } from "~/lib/nodeGraph";
    import Interpreter from "~/lib/interpreter";
    import { onDestroy, onMount } from "svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import type { NodeGraphUpdatePayload } from "~/lib/payload";
    import { invoke } from "@tauri-apps/api/core";

    const { screenToFlowPosition, updateNodeData, getNode } = useSvelteFlow();

    export let nodeTypes: NodeTypes | undefined;
    export let edgeTypes: EdgeTypes | undefined;
    export let nodes: Writable<Node[]>;
    export let edges: Writable<Edge[]>;
    let updateUnlisten: UnlistenFn | undefined;

    onMount(async () => {
        $interpreter = new Interpreter();
        $interpreter.start();
        console.log("interpreter", $interpreter);

        updateUnlisten = await listen<NodeGraphUpdatePayload>(
            "node-graph-update",
            (event) => {
                Object.entries(event.payload.devices).forEach(
                    ([id, device]) => {
                        let node = $nodes.find((node) => node.id === id);
                        if (node) {
                            switch (node.type) {
                                case "distance":
                                    const distance = device as {
                                        DistanceSensor: {
                                            distance: number;
                                            size: number;
                                        };
                                    };
                                    getNode(id)?.data.distance.set(distance.DistanceSensor.distance);
                                    break;
                                case "light_sensor":
                                    const light = device as {
                                        LightSensor: {
                                            darkness: number;
                                        };
                                    };
                                    node.data = light.LightSensor;
                                    break;
                            }
                            node.data = device;
                        }
                    },
                );
            },
        );

        // setInterval(() => {
        //     console.log("grpah", $nodes);
        // }, 100);
    });

    onDestroy(() => {
        updateUnlisten?.();
    });

    $: {
        let nodeGraph = svelteFlowToNodeGraph($nodes, $edges);
        $interpreter?.update(nodeGraph);
    }
</script>

<SvelteFlow
    {nodeTypes}
    {edgeTypes}
    {nodes}
    {edges}
    fitView
    fitViewOptions={{ maxZoom: 1.0 }}
>
    <Background variant={BackgroundVariant.Lines} />
    <Controls />
    <MiniMap />
</SvelteFlow>
