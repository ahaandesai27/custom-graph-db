use core::fmt;

use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::graph::node::properties::property_map::PropertyMap;

pub type NodeId = u64;

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub label: String,
    pub property_map: PropertyMap,
}


impl Node {
    pub fn new(id: u64, label: String) -> Self {
        Self {
            id,
            label,
            property_map: HashMap::new(),
        }
    }
}

impl fmt::Display for Node {
    // implementing display trait for 'Node'

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formatter object in fmt, '_ is an anonymous lifetime
        let serialized_map = serde_json::to_string(&self.property_map).unwrap();
        write!(f, "Node(label={}, properties={})", self.label, serialized_map)
        // later , show properties too 
    }
}

pub mod crud;
pub mod properties;