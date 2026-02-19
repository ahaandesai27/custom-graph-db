use dashmap::{DashMap, DashSet};

use crate::{graph::{edge::{Edge, EdgeId}, idgen::IdGenerator, node::{Node, NodeId}}, utils::shared::Shared};

pub struct Graph {
    pub nodes: DashMap<NodeId, Shared<Node>>,                 // index by id 
    pub edges: DashMap<EdgeId, Shared<Edge>>,

    // Label storage - These are only meant for getting nodes 
    pub label_node_index: DashMap<String, DashSet<NodeId>>,
    pub label_edge_index: DashMap<String, DashSet<EdgeId>>,

    // ID generator - uses Atomic64
    pub node_idgen: IdGenerator,
    pub edge_idgen: IdGenerator,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: DashMap::new(),
            edges: DashMap::new(),
            label_node_index: DashMap::new(),
            label_edge_index: DashMap::new(),
            node_idgen: IdGenerator::new(1),
            edge_idgen: IdGenerator::new(1)
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }


}

pub mod edge;
pub mod idgen;
pub mod node;
pub mod traverse;






