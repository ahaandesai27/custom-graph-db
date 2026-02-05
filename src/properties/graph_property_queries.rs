use std::collections::HashMap;

use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::properties::types::property_query::PropertyQueryMap;
use serde_json::Value;
use uuid::Uuid;

impl Graph {
    pub fn find_nodes_satisfying_label_and_property(
        &self,
        label: &str,
        properties: &PropertyQueryMap,
    ) -> Vec<&Node> {
        let ids: Vec<Uuid> = self.label_node_index
            .get(label)
            .into_iter()
            .flatten()
            .filter(|id| {
                self.get_node(**id)
                    .is_some_and(|node| node.is_satisfying_property(properties))
            })
            .copied()
            .collect();
    
        let result: Vec<&Node> = ids
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .collect();
    
        return result;
    }

    pub fn find_nodes_satisfying_property(&self, properties: PropertyQueryMap) -> Vec<&Node> {
        // needs to go to index hashmap
        let nodes: Vec<&Node> = self.nodes
            .values()               // values gives us references to node 
            .collect();

        let results: Vec<&Node> = nodes.into_iter()
            .filter(|node| node.is_satisfying_property(&properties))
            .collect();
        
        results 

        // note: rust auto dereferences - BUT ONLY FOR METHOD CALLS 
    }
}
