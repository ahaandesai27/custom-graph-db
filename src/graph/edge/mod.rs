use core::fmt;
use crate::graph::node::{NodeId};

#[derive(Hash, Eq, PartialEq)]
pub struct Edge {
    pub src: NodeId,
    pub dst: NodeId,
    pub label: String,
}

impl Edge {
    pub fn new(src: NodeId, dst: NodeId, label: String) -> Self {
        Self { src, dst, label }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Edge(label={})", self.label)
    }
}

pub mod crud;