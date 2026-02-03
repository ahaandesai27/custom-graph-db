
use std::collections::HashMap;

use crate::graph::graph::Graph;
use crate::graph::node::Node;
use uuid::Uuid;
use serde_json::Value;

impl Graph {
    pub fn find_nodes_by_label_and_property(&self, properties: HashMap<String, Value>) {
        // can go to label hashmap 
    }

    pub fn find_nodes_by_property(&self, properties: HashMap<String, Value>) {
        // needs to go to index hashmap 
    }
}