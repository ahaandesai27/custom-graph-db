use core::fmt;
use crate::graph::node::{NodeId};

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
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node(label={})", self.label)
    }
}

pub mod crud;