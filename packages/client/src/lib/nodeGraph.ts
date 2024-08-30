import { useSvelteFlow, type Edge as FlowEdge, type Node as FlowNode } from "@xyflow/svelte"

export interface Brain {
    port_1?: string | undefined,
    port_2?: string | undefined,
    port_3?: string | undefined,
    port_4?: string | undefined,
    port_5?: string | undefined,
    port_6?: string | undefined,
    port_7?: string | undefined,
    port_8?: string | undefined,
    port_9?: string | undefined,
    port_10?: string | undefined,
    port_11?: string | undefined,
    port_12?: string | undefined,
    port_13?: string | undefined,
    port_14?: string | undefined,
    port_15?: string | undefined,
    port_16?: string | undefined,
    port_17?: string | undefined,
    port_18?: string | undefined,
    port_19?: string | undefined,
    port_20?: string | undefined,
    port_21?: string | undefined
}

export interface Input {
    source_id: string,
    target_handle_id: string,
}

export interface DistanceSensor {
    distance?: number | undefined,
    size?: number | undefined
}

export interface LightSensor {
    darkness?: number | undefined
}

export interface Value {
    value?: number | undefined
}

export interface Math {
    operation: string,
    lhs?: number | undefined
    rhs?: number | undefined
}

export interface Node {
    data: {
        type: "DistanceSensor" | "LightSensor" | "Value" | "Math",
        data: DistanceSensor | LightSensor | Value | Math
    } | "Time"
    inputs?: Input[] | undefined
}

export interface NodeGraph { brain: Brain, nodes: { [key: string]: Node } }

export function convertNodeType(flowType: string): "DistanceSensor" | "LightSensor" | "Value" | "Math" | "Time" | undefined {
    switch (flowType) {
        case "light_sensor": {
            return "LightSensor"
        }
        case "time": {
            return "Time"
        }
        case "math": {
            return "Math"
        }
        case "value": {
            return "Value"
        }
        case "distance": {
            return "DistanceSensor"
        }
        case "time": {
            return "Time"
        }
    }
}

export function convertNodeData(flowData: any, type: "DistanceSensor" | "LightSensor" | "Value" | "Math"): DistanceSensor | LightSensor | Value | Math {
    switch (type) {
        case "DistanceSensor": {
            return {
                distance: flowData.distance,
                size: flowData.size
            }
        }
        case "LightSensor": {
            return {
                darkness: flowData.darkness
            }
        }
        case "Value": {
            return {
                value: flowData.value
            }
        }
        case "Math": {
            return {
                operation: flowData.operation,
                lhs: flowData.lhs,
                rhs: flowData.rhs
            }
        }
    }
}

function setBrainPort(port: number, incoming_id: string, brain: Brain) {
    let portName = `port_${port}` as keyof Brain;
    brain[portName] = incoming_id;
}

function buildNodes(root: FlowNode, nodes: FlowNode[], edges: FlowEdge[], acc: { id: string, node: Node }[]) {
    const inputs = edges.filter(edge => edge.target === root.id).map(edge => { return { target: edge.targetHandle, node: nodes.find((node => node.id === edge.source)) }; })
        .filter((node): node is { target: string, node: FlowNode } => node.node !== undefined && node.target !== undefined);
    const input_ids = inputs.map(input => { return { handle: input.target, id: input.node.id } });

    inputs.forEach(input => {
        buildNodes(input.node, nodes, edges, acc);
    });

    if (!root.type) {
        return;
    }
    const type = convertNodeType(root.type);
    if (type && !acc.find(node => node.id === root.id)) {
        const data: Node["data"] = type === "Time" ? "Time" : {
            type,
            data: convertNodeData(root.data, type)
        };
        const node: Node = {
            data,
            inputs: input_ids.map((id, idx) => ({
                source_id: id.id,
                target_handle_id: id.handle,
            }))
        };

        acc.push({ id: root.id, node });
    }
}

export function svelteFlowToNodeGraph(nodes: FlowNode[], edges: FlowEdge[]): NodeGraph {
    let brain: Brain = {};

    const roots = edges.filter(edge => edge.target === "brain").map(edge => {
        const portId = edge.targetHandle?.match(/\d+$/);
        if (portId) {
            setBrainPort(parseInt(portId[0]), edge.source, brain);
        }
        return nodes.find(node => node.id === edge.source);
    }).filter((node): node is FlowNode => node !== undefined);

    let graphNodes: { id: string, node: Node }[] = [];
    roots.forEach(root => {
        buildNodes(root, nodes, edges, graphNodes);
    });
    const graphNodesMap = graphNodes.reduce<{ [key: string]: Node }>((acc, node) => {
        acc[node.id] = node.node;
        return acc;
    }, {});

    return { brain, nodes: graphNodesMap };
}
