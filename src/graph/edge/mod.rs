use core::fmt;

use uuid::Uuid;
use crate::graph::node::{Node, NodeId};

pub type EdgeId = u64;
pub struct Edge {
    pub id: EdgeId,
    pub src: NodeId,
    pub dst: NodeId,
    pub label: String,
}

impl Edge {
    pub fn new(id: u64, src: NodeId, dst: NodeId, label: String) -> Self {
        Self { id, src, dst, label }
    }

    pub fn from_nodes(id: u64, src: &Node, dst: &Node, label: String) -> Self {
        // Reference & is needed
        // Otherwise the acutal object is passed to the function and the function owns the object
        // The objects would be destroyed after the function ends due to the borrow checker
        Self {
            id,
            src: src.id,
            dst: dst.id,
            label,
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(label={})", self.label)
    }
}

pub mod crud;