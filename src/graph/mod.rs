use std::collections::HashMap;
use crate::graph::{edge::{Edge, EdgeId}, idgen::IdGenerator, node::{Node, NodeId}};

pub struct Graph {
    pub nodes: HashMap<NodeId, Node>,                 // index by id 
    pub edges: HashMap<EdgeId, Edge>,

    // Label storage 
    pub label_node_index: HashMap<String, Vec<NodeId>>,
    pub label_edge_index: HashMap<String, Vec<EdgeId>>,

    // ID generator
    pub node_idgen: IdGenerator,
    pub edge_idgen: IdGenerator,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            label_node_index: HashMap::new(),
            label_edge_index: HashMap::new(),
            node_idgen: IdGenerator::new(1),
            edge_idgen: IdGenerator::new(1)
        }
    }
}

pub mod edge;
pub mod idgen;
pub mod node;
pub mod traverse;






