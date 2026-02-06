use std::collections::HashMap;
use uuid::Uuid;

use crate::graph::{edge::Edge, node::Node};

pub type NodeId = Uuid;
pub type EdgeId = Uuid;

pub struct Graph {
    pub nodes: HashMap<NodeId, Node>,                 // index by id 
    pub edges: HashMap<EdgeId, Edge>,

    // Adjacency indices 
    pub out_index: HashMap<NodeId, Vec<EdgeId>>,
    pub in_index: HashMap<NodeId, Vec<EdgeId>>,

    // Label storage 
    pub label_node_index: HashMap<String, Vec<NodeId>>,
    pub label_edge_index: HashMap<String, Vec<EdgeId>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            out_index: HashMap::new(),
            in_index: HashMap::new(),
            label_node_index: HashMap::new(),
            label_edge_index: HashMap::new(),
        }
    }
}

pub mod edge;
pub mod node;






