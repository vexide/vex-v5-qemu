use miette::Diagnostic;
use thiserror::Error;

use crate::{
    ast::{AdiDeviceNode, DataInput, DataNode, DeviceInput, Node, SmartDeviceNode},
    parser::{NodeType, Operation},
};

/// Represents a device node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Device {
    Distance { distance: f32, size: f32 },
}
impl Into<NodeType> for Device {
    fn into(self) -> NodeType {
        match self {
            Device::Distance { distance, size } => NodeType::Distance {
                distance: Some(distance),
                size: Some(size),
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

pub fn evaluate_data(node: &DataNode) -> f32 {
    match node {
        DataNode::Math {
            operation,
            lhs,
            rhs,
        } => {
            let lhs = match lhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node)
            };

            let rhs = match rhs {
                DataInput::Value(value) => *value,
                DataInput::DataNode(node) => evaluate_data(node),
            };

            match operation {
                Operation::Add => lhs + rhs,
                Operation::Subtract => lhs - rhs,
                Operation::Multiply => lhs * rhs,
                Operation::Divide => lhs / rhs,
            }
        }
        DataNode::Value(value) => *value,
    }
}

pub fn evaluate_smart_device(node: &SmartDeviceNode) -> Device {
    match node {
        SmartDeviceNode::Distance { distance, size } => {
            let distance = match distance {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node),
                },
            };
            let size = match size {
                DeviceInput::Value(value) => match value {
                    DataInput::Value(value) => *value,
                    DataInput::DataNode(node) => evaluate_data(node),
                },
            };

            Device::Distance { distance, size }
        }
    }
}

fn evaluate_adi_device(_node: &AdiDeviceNode) -> Device {
    todo!()
}

/// Evaluates an AST and returns the output.
pub fn evaluate(ast: &Node) -> InterpereterOutput {
    match ast {
        Node::DataNode(node) => InterpereterOutput::Value(evaluate_data(node)),
        Node::SmartDeviceNode(node) => InterpereterOutput::Device(evaluate_smart_device(node)),
        Node::AdiDeviceNode(node) => InterpereterOutput::Device(evaluate_adi_device(node)),
    }
}
