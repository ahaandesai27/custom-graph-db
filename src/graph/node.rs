use core::fmt;

use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub property_map: HashMap<String, Value>,
}

impl Node {
    pub fn new(label: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            label: label,
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