use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    ast::{
        AdiDeviceNode, AdiDeviceNodeData, Brain, DataInput, DataNode, DataNodeData, DeviceInput,
        Node, NodeData, SmartDeviceNode, SmartDeviceNodeData,
    },
    parser::{NodeType, Operation},
};

/// Represents a device node.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Device {
    DistanceSensor { distance: f32, size: f32 },
    LightSensor { darkness: f32 },
}
impl From<Device> for NodeType {
    fn from(val: Device) -> Self {
        match val {
            Device::DistanceSensor { distance, size } => NodeType::DistanceSensor {
                distance: Some(distance),
                size: Some(size),
            },
            Device::LightSensor { darkness } => NodeType::LightSensor {
                darkness: Some(darkness),
            },
        }
    }
}

/// Represents an evaluated output from an AST.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpreterOutput {
    /// The AST evaluated down to a single value.
    Value(f32),
    /// The AST evaluated down to a device node.
    Device(Device),
}

pub struct InterpreterContext {
    time: f32,
    roots: Vec<String>,
    nodes: BTreeMap<String, NodeData>,
    evaluated: BTreeMap<String, InterpreterOutput>,
}
impl InterpreterContext {
    pub fn from_brain(time: f32, brain: &Brain) -> Self {
        let root_nodes = brain
            .smart_ports()
            .into_values()
            .map(|port| Node::SmartDeviceNode(port.clone()))
            .chain(
                brain
                    .adi_ports()
                    .into_values()
                    .map(|port| Node::AdiDeviceNode(port.clone())),
            )
            .collect();

        Self::from_root_nodes(time, root_nodes)
    }

    pub fn from_root_nodes(time: f32, root_nodes: Vec<Node>) -> Self {
        let root_node_ids = root_nodes.iter().map(|node| node.id().to_owned()).collect();
        let mut nodes = BTreeMap::new();
        for node in root_nodes {
            nodes.insert(node.id().to_owned(), node.into_data());
        }

        Self {
            roots: root_node_ids,
            time,
            nodes,
            evaluated: BTreeMap::new(),
        }
    }

    pub fn new(time: f32) -> Self {
        Self {
            time,
            roots: Vec::new(),
            nodes: BTreeMap::new(),
            evaluated: BTreeMap::new(),
        }
    }

    pub fn update_time(&mut self, time: f32) {
        self.time = time;
        self.evaluated.clear();
    }

    pub fn root_nodes(&self) -> Vec<Node> {
        self.roots
            .iter()
            .filter_map(|id| {
                self.nodes
                    .get(id)
                    .map(|node| Node::from_data(node.clone(), id.to_owned()))
            })
            .collect()
    }

    pub fn evaluated_nodes(&self) -> &BTreeMap<String, InterpreterOutput> {
        &self.evaluated
    }
}

pub fn evaluate_data(node: &DataNode, ctx: &mut InterpreterContext) -> f32 {
    if let Some(InterpreterOutput::Value(value)) = ctx.evaluated.get(&node.id) {
        return *value;
    }

    let result = match &node.data {
        DataNodeData::Math {
            operation,
            lhs,
            rhs,
        } => {
            let lhs = match lhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node, ctx),
            };

            let rhs = match rhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node, ctx),
            };

            match operation {
                Operation::Add => lhs + rhs,
                Operation::Subtract => lhs - rhs,
                Operation::Multiply => lhs * rhs,
                Operation::Divide => lhs / rhs,
            }
        }
        DataNodeData::Value(value) => *value,
        DataNodeData::Time => ctx.time,
    };

    ctx.evaluated
        .insert(node.id.to_owned(), InterpreterOutput::Value(result));
    result
}

pub fn evaluate_smart_device(node: &SmartDeviceNode, ctx: &mut InterpreterContext) -> Device {
    if let Some(InterpreterOutput::Device(device)) = ctx.evaluated.get(&node.id) {
        return *device;
    }

    let result = match &node.data {
        SmartDeviceNodeData::Distance { distance, size } => {
            let distance = match distance {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, ctx),
                },
            };
            let size = match size {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, ctx),
                },
            };

            Device::DistanceSensor { distance, size }
        }
    };

    ctx.evaluated
        .insert(node.id.to_owned(), InterpreterOutput::Device(result));
    result
}

pub fn evaluate_adi_device(node: &AdiDeviceNode, ctx: &mut InterpreterContext) -> Device {
    if let Some(InterpreterOutput::Device(device)) = ctx.evaluated.get(&node.id) {
        return *device;
    }

    let result = match &node.data {
        AdiDeviceNodeData::Light { darkness } => {
            let darkness = match darkness {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, ctx),
                },
            };

            Device::LightSensor { darkness }
        }
    };

    ctx.evaluated
        .insert(node.id.to_owned(), InterpreterOutput::Device(result));
    result
}

pub fn evaluate(node: &Node, ctx: &mut InterpreterContext) -> Option<InterpreterOutput> {
    if ctx.evaluated.contains_key(node.id()) {
        return Some(ctx.evaluated[node.id()]);
    }

    let result = match node {
        Node::DataNode(node) => InterpreterOutput::Value(evaluate_data(node, ctx)),
        Node::SmartDeviceNode(node) => InterpreterOutput::Device(evaluate_smart_device(node, ctx)),
        Node::AdiDeviceNode(node) => InterpreterOutput::Device(evaluate_adi_device(node, ctx))
    };
    ctx.evaluated.insert(node.id().to_owned(), result);

    Some(result)
}
