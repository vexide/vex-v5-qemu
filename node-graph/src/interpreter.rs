use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    ast::{AdiDeviceNode, DataInput, DataNode, DeviceInput, Node, SmartDeviceNode},
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
pub enum InterpereterOutput {
    /// The AST evaluated down to a single value.
    Value(f32),
    /// The AST evaluated down to a device node.
    Device(Device),
}

#[derive(Diagnostic, Error, Debug)]
pub enum InterpereterError {
    #[error("Incorrect node input type")]
    #[diagnostic(code(node_graph::interpreter::evaluate_error))]
    //TODO: more info
    EvaluateError,
}

pub fn evaluate_data(node: &DataNode, time: f32) -> f32 {
    match node {
        DataNode::Math {
            operation,
            lhs,
            rhs,
        } => {
            let lhs = match lhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node, time),
            };

            let rhs = match rhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node, time),
            };

            match operation {
                Operation::Add => lhs + rhs,
                Operation::Subtract => lhs - rhs,
                Operation::Multiply => lhs * rhs,
                Operation::Divide => lhs / rhs,
            }
        }
        DataNode::Value(value) => *value,
        DataNode::Time => time,
    }
}

pub fn evaluate_smart_device(node: &SmartDeviceNode, time: f32) -> Device {
    match node {
        SmartDeviceNode::Distance { distance, size } => {
            let distance = match distance {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, time),
                },
            };
            let size = match size {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, time),
                },
            };

            Device::DistanceSensor { distance, size }
        }
    }
}

pub fn evaluate_adi_device(node: &AdiDeviceNode, time: f32) -> Device {
    match node {
        AdiDeviceNode::Light { darkness } => {
            let darkness = match darkness {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node, time),
                },
            };

            Device::LightSensor { darkness }
        }
    }
}

/// Evaluates an AST and returns the output.
pub fn evaluate(ast: &Node, time: f32) -> InterpereterOutput {
    match ast {
        Node::DataNode(node) => InterpereterOutput::Value(evaluate_data(node, time)),
        Node::SmartDeviceNode(node) => {
            InterpereterOutput::Device(evaluate_smart_device(node, time))
        }
        Node::AdiDeviceNode(node) => InterpereterOutput::Device(evaluate_adi_device(node, time)),
    }
}
