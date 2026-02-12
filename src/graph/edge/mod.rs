use core::fmt;

use uuid::Uuid;
use crate::graph::node::{Node, NodeId};

pub type EdgeId = Uuid;
pub struct Edge {
    pub id: EdgeId,
    pub src: NodeId,
    pub dst: NodeId,
    pub label: String,
}

impl Edge {
    pub fn new(src: NodeId, dst: NodeId, label: String) -> Self {
        Self { id: Uuid::now_v7(), src, dst, label }
    }

    pub fn from_nodes(src: &Node, dst: &Node, label: String) -> Self {
        // Reference & is needed
        // Otherwise the acutal object is passed to the function and the function owns the object
        // The objects would be destroyed after the function ends due to the borrow checker
        Self {
            id: Uuid::now_v7(),
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