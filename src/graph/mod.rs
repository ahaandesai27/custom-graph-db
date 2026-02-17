use std::collections::{HashMap, HashSet};
use dashmap::DashMap;

use crate::{graph::{edge::{Edge, EdgeId}, idgen::IdGenerator, node::{Node, NodeId}}, utils::shared::Shared};

pub struct Graph {
    pub nodes: DashMap<NodeId, Shared<Node>>,                 // index by id 
    pub edges: DashMap<EdgeId, Shared<Edge>>,

    // Label storage - These are only meant for getting nodes 
    pub label_node_index: HashMap<String, HashSet<NodeId>>,
    pub label_edge_index: HashMap<String, HashSet<EdgeId>>,

    // ID generator - uses Atomic64
    pub node_idgen: IdGenerator,
    pub edge_idgen: IdGenerator,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: DashMap::new(),
            edges: DashMap::new(),
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






