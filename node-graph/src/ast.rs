use std::collections::BTreeMap;

use miette::Diagnostic;
use thiserror::Error;

use crate::parser::{NodeGraph, Operation};

#[derive(Debug, Clone, PartialEq)]
pub enum DataInput {
    Value(f32),
    DataNode(Box<DataNode>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataNode {
    Math {
        operation: Operation,
        lhs: DataInput,
        rhs: DataInput,
    },
    Value(f32),
    Time,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceInput {
    Value(DataInput),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmartDeviceNode {
    Distance {
        distance: DeviceInput,
        size: DeviceInput,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AdiDeviceNode {
    Light { darkness: DeviceInput },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    DataNode(DataNode),
    SmartDeviceNode(SmartDeviceNode),
    AdiDeviceNode(AdiDeviceNode),
}

#[derive(Error, Debug)]
#[error("Port at index: {given_index} does not exist.")]
pub struct SetPortError {
    given_index: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brain {
    pub ports: [Option<SmartDeviceNode>; 21],

    pub adi_a: Option<AdiDeviceNode>,
    pub adi_b: Option<AdiDeviceNode>,
    pub adi_c: Option<AdiDeviceNode>,
    pub adi_d: Option<AdiDeviceNode>,
    pub adi_e: Option<AdiDeviceNode>,
    pub adi_f: Option<AdiDeviceNode>,
    pub adi_g: Option<AdiDeviceNode>,
    pub adi_h: Option<AdiDeviceNode>,
}
impl Brain {
    pub fn new() -> Self {
        Self {
            ports: std::array::from_fn(|_| None),

            adi_a: None,
            adi_b: None,
            adi_c: None,
            adi_d: None,
            adi_e: None,
            adi_f: None,
            adi_g: None,
            adi_h: None,
        }
    }
    pub fn smart_ports(&self) -> BTreeMap<u8, &SmartDeviceNode> {
        self.ports
            .iter()
            .enumerate()
            .filter_map(|(i, v)| v.as_ref().map(|v| (i as u8, v)))
            .collect()
    }
    pub fn adi_ports(&self) -> BTreeMap<char, &AdiDeviceNode> {
        [
            ('a', &self.adi_a),
            ('b', &self.adi_b),
            ('c', &self.adi_c),
            ('d', &self.adi_d),
            ('e', &self.adi_e),
            ('f', &self.adi_f),
            ('g', &self.adi_g),
            ('h', &self.adi_h),
        ]
        .into_iter()
        .filter_map(|(i, v)| v.as_ref().map(|v| (i, v)))
        .collect()
    }

    pub fn set_smart_port(&mut self, port: u8, node: SmartDeviceNode) -> Result<(), SetPortError> {
        if !(0..=21).contains(&port) {
            return Err(SetPortError {
                given_index: format!("'{}'", port),
            });
        }

        self.ports[port as usize] = Some(node);

        Ok(())
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Diagnostic, Debug)]
#[error("Failed to convert node graph to AST")]
pub enum AstConversionError {
    #[error("Cannot find node with id {node_id}")]
    #[diagnostic(code(node_graph::ast::node_not_found))]
    NodeNotFound { node_id: String },
    #[error("An invalid connection was found on node with id: {node_id}.\nExpected a node of type {expected} but found a node of type {found}")]
    #[diagnostic(code(node_graph::ast::incorrect_node_type))]
    IncorrectNodeType {
        node_id: String,
        expected: String,
        found: String,
    },
    #[error("An invalid target handle id was found on node with id: {node_id}.\nExpected one of: {expected} but found an id of {found}")]
    #[diagnostic(code(node_graph::ast::incorrect_target_id))]
    IncorrectTargetHandleId {
        node_id: String,
        expected: String,
        found: String,
    },
    #[error("A duplicate target handle connection was found on node with id: {node_id}.\nTarget handle id: {target_handle_id}")]
    #[diagnostic(code(node_graph::ast::duplicate_target_id))]
    DuplicateTargetHandleId {
        node_id: String,
        target_handle_id: String,
    },
}

fn clean_inputs(
    id: &str,
    keys: &[&str],
    inputs: BTreeMap<String, Node>,
) -> Result<BTreeMap<String, Node>, AstConversionError> {
    if keys.is_empty() && inputs.is_empty() {
        return Ok(inputs);
    }

    let mut filetered_inputs = BTreeMap::new();
    for (key, node) in inputs.into_iter() {
        if !keys.contains(&key.as_str()) {
            return Err(AstConversionError::IncorrectTargetHandleId {
                node_id: id.to_owned(),
                expected: keys.join(", "),
                found: key,
            });
        }

        filetered_inputs.insert(key, node);
    }

    Ok(filetered_inputs)
}

fn handle_device_input(
    id: &str,
    input: Option<&Node>,
    fallback: f32,
) -> Result<DeviceInput, AstConversionError> {
    match input {
        Some(Node::DataNode(DataNode::Value(value))) => {
            Ok(DeviceInput::Value(DataInput::Value(*value)))
        }
        Some(Node::DataNode(node)) => Ok(DeviceInput::Value(DataInput::DataNode(Box::new(
            node.clone(),
        )))),
        Some(_) => Err(AstConversionError::IncorrectNodeType {
            node_id: id.to_string(),
            expected: "DataNode".to_owned(),
            //TODO: fix this
            found: "TODO".to_owned(),
        }),
        None => Ok(DeviceInput::Value(DataInput::Value(fallback))),
    }
}
fn handle_data_input(
    id: &str,
    input: Option<&Node>,
    fallback: f32,
) -> Result<DataInput, AstConversionError> {
    match input {
        Some(Node::DataNode(node)) => Ok(DataInput::DataNode(Box::new(node.clone()))),
        Some(_) => Err(AstConversionError::IncorrectNodeType {
            node_id: id.to_string(),
            expected: "DataNode".to_owned(),
            found: "SmartDeviceNode".to_owned(),
        }),
        None => Ok(DataInput::Value(fallback)),
    }
}

fn build_ast(graph: &NodeGraph, id: &str) -> Result<Node, AstConversionError> {
    let node = graph
        .nodes
        .get(id)
        .ok_or(AstConversionError::NodeNotFound {
            node_id: id.to_owned(),
        })?;

    let mut inputs: BTreeMap<String, Node> = BTreeMap::new();
    //TODO: handle output handle ids
    for input in node.inputs.iter().flatten() {
        let source_node = build_ast(graph, &input.source_id)?;

        if inputs.contains_key(&input.target_handle_id) {
            return Err(AstConversionError::DuplicateTargetHandleId {
                node_id: id.to_owned(),
                target_handle_id: input.target_handle_id.clone(),
            });
        }

        inputs.insert(input.target_handle_id.clone(), source_node);
    }


    match &node.data {
        crate::parser::NodeType::DistanceSensor { distance, size } => {
            let inputs = clean_inputs(id, &["distance", "size"], inputs)?;
            let distance =
                handle_device_input(id, inputs.get("distance"), distance.unwrap_or_default())?;
            let size = handle_device_input(id, inputs.get("size"), size.unwrap_or_default())?;
            Ok(Node::SmartDeviceNode(SmartDeviceNode::Distance {
                distance,
                size,
            }))
        }
        crate::parser::NodeType::Value { value } => {
            clean_inputs(id, &[], inputs)?;

            Ok(Node::DataNode(DataNode::Value(*value)))
        }
        crate::parser::NodeType::Math {
            operation,
            lhs,
            rhs,
        } => {
            let inputs = clean_inputs(id, &["lhs", "rhs"], inputs)?;
            let lhs = handle_data_input(id, inputs.get("lhs"), lhs.unwrap_or_default())?;
            let rhs = handle_data_input(id, inputs.get("rhs"), rhs.unwrap_or_default())?;

            Ok(Node::DataNode(DataNode::Math {
                operation: *operation,
                lhs,
                rhs,
            }))
        }
        crate::parser::NodeType::LightSensor { darkness } => {
            let inputs = clean_inputs(id, &["darkness"], inputs)?;
            let darkness =
                handle_device_input(id, inputs.get("darkness"), darkness.unwrap_or_default())?;

            Ok(Node::AdiDeviceNode(AdiDeviceNode::Light { darkness }))
        },
        crate::parser::NodeType::Time => {
            clean_inputs(id, &[], inputs)?;

            Ok(Node::DataNode(DataNode::Time))
        },
    }
}

pub fn node_graph_to_ast(graph: &NodeGraph) -> Result<Brain, AstConversionError> {
    let _ids = graph.nodes.keys().collect::<Vec<_>>();
    let mut brain = crate::ast::Brain::new();

    for (port, source_id) in graph.brain.smart_ports() {
        let source_node = build_ast(graph, source_id)?;
        if let Node::SmartDeviceNode(source_node) = source_node {
            // This will never panic because we are iterating over the smart ports of the
            // brain
            brain.set_smart_port(port, source_node).unwrap();
        } else {
            return Err(AstConversionError::IncorrectNodeType {
                node_id: source_id.clone(),
                expected: "SmartDeviceNode".to_owned(),
                found: "DataNode".to_owned(),
            });
        }
    }

    for (port, source_id) in graph.brain.adi_ports() {
        let source_node = build_ast(graph, source_id)?;
        if let Node::AdiDeviceNode(source_node) = source_node {
            match port {
                'a' => brain.adi_a = Some(source_node),
                'b' => brain.adi_b = Some(source_node),
                'c' => brain.adi_c = Some(source_node),
                'd' => brain.adi_d = Some(source_node),
                'e' => brain.adi_e = Some(source_node),
                'f' => brain.adi_f = Some(source_node),
                'g' => brain.adi_g = Some(source_node),
                'h' => brain.adi_h = Some(source_node),
                _ => unreachable!(),
            }
        } else {
            return Err(AstConversionError::IncorrectNodeType {
                node_id: source_id.clone(),
                expected: "AdiDeviceNode".to_owned(),
                found: "DataNode".to_owned(),
            });
        }
    }

    Ok(brain)
}
