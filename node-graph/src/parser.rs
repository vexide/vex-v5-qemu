use std::collections::BTreeMap;

use miette::{Diagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeGraph {
    pub brain: Brain,
    pub nodes: BTreeMap<String, Node>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Brain {
    pub port_1: Option<String>,
    pub port_2: Option<String>,
    pub port_3: Option<String>,
    pub port_4: Option<String>,
    pub port_5: Option<String>,
    pub port_6: Option<String>,
    pub port_7: Option<String>,
    pub port_8: Option<String>,
    pub port_9: Option<String>,
    pub port_10: Option<String>,
    pub port_11: Option<String>,
    pub port_12: Option<String>,
    pub port_13: Option<String>,
    pub port_14: Option<String>,
    pub port_15: Option<String>,
    pub port_16: Option<String>,
    pub port_17: Option<String>,
    pub port_18: Option<String>,
    pub port_19: Option<String>,
    pub port_20: Option<String>,
    pub port_21: Option<String>,

    pub adi_a: Option<String>,
    pub adi_b: Option<String>,
    pub adi_c: Option<String>,
    pub adi_d: Option<String>,
    pub adi_e: Option<String>,
    pub adi_f: Option<String>,
    pub adi_g: Option<String>,
    pub adi_h: Option<String>,
}
impl Brain {
    pub fn smart_ports(&self) -> Vec<(u8, &String)> {
        [
            &self.port_1,
            &self.port_2,
            &self.port_3,
            &self.port_4,
            &self.port_5,
            &self.port_6,
            &self.port_7,
            &self.port_8,
            &self.port_9,
            &self.port_10,
            &self.port_11,
            &self.port_12,
            &self.port_13,
            &self.port_14,
            &self.port_15,
            &self.port_16,
            &self.port_17,
            &self.port_18,
            &self.port_19,
            &self.port_20,
            &self.port_21,
        ]
        .into_iter()
        .enumerate()
        .filter_map(|(i, port)| port.as_ref().map(|port| (i as u8, port)))
        .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Node {
    pub inputs: Option<Vec<Input>>,
    pub data: NodeType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Input {
    pub source_id: String,
    pub target_handle_id: String,
    pub source_handle_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum NodeType {
    Distance {
        distance: Option<f32>,
        size: Option<f32>,
    },
    Value {
        value: f32,
    },
    Math {
        operation: Operation,
        lhs: Option<f32>,
        rhs: Option<f32>,
    },
}

#[derive(Error, Diagnostic, Debug)]
#[error("Failed to parse node graph: {message}")]
#[diagnostic(code(node_graph::parser::parse_error))]
pub struct ParseError {
    message: String,

    #[source_code]
    source_code: String,
    #[label("Error occured here")]
    span: Option<SourceSpan>,
}

pub fn parse_node_graph(input: &str) -> Result<NodeGraph, ParseError> {
    toml::from_str(input).map_err(|e| ParseError {
        span: e
            .span()
            .map(|span| SourceSpan::new(span.start.into(), span.len())),
        message: e.message().to_string(),
        source_code: input.to_string(),
    })
}

pub fn serialize_node_graph(graph: &NodeGraph) -> Result<String, toml::ser::Error> {
    toml::to_string(graph)
}