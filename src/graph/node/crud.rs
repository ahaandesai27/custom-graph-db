// CRUD on graph nodes

use std::collections::HashSet;

use dashmap::DashSet;

use crate::{
    graph::{
        Graph,
        node::{
            Node, NodeId,
            properties::{property_map::PropertyMap, property_query_map::PropertyQueryMap},
        },
    },
    utils::shared::{Shared, shared},
};

impl Graph {
    pub fn add_node(&self, label: &str, properties: &PropertyMap) -> NodeId {
        let id = self.node_idgen.next_id();
        let node = shared(Node::new(id, label.to_string(), properties.clone()));

        self.nodes.insert(id, node);

        let entry = self
            .label_node_index
            .entry(label.to_string())
            .or_insert_with(DashSet::new);

        entry.insert(id);

        id
    }

    pub fn get_node(&self, id: NodeId) -> Option<Shared<Node>> {
        self.nodes.get(&id).map(|entry| entry.clone())

        // it returns a guard
        // we must clone the arc  - otherwise the reference goes out of scope
        // else the guard to the node dies after we leave scope

        // if lock is needed, lock the node on return (in the function that calls) and update
    }

    pub fn get_node_ids_by_label(&self, label: &str) -> Option<HashSet<NodeId>> {
        let ids: HashSet<NodeId> = self
            .label_node_index
            .get(label)
            .map(|guard| guard.iter().map(|r| *r.key()).collect())
            .unwrap_or_default();

        Some(ids)

        // must clone data because reference might outlive guard
    }

    pub fn get_nodes_satisfying_label_and_property(
        &self,
        label: Option<&str>,
        properties: &PropertyQueryMap,
    ) -> Vec<Shared<Node>> {
        let ids: HashSet<NodeId> = match label {
            Some(l) => self.get_node_ids_by_label(l).unwrap(),
            None => self.nodes.iter().map(|e| *e.key()).collect(),
        };

        ids.iter()
            .filter_map(|id| {
                self.nodes.get(id).and_then(|entry| {
                    let node = entry.value();
                    let guard = node.read().unwrap();
                    if guard.is_satisfying_property(properties) {
                        Some(node.clone())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn delete_node(&self, id: NodeId) {
        if let Some(entry) = self.nodes.remove(&id) {
            let node = entry.1;

            let label = {
                // lock dropped early 
                let guard = node.read().unwrap();
                guard.label.clone()
            };

            if let Some(label_set) = self.label_node_index.get(&label) {
                label_set.remove(&id);
            }

            self.edges.retain(|edge| edge.src != id && edge.dst != id);

            for entry in self.label_edge_index.iter() {
                entry
                    .value()
                    .retain(|edge| edge.src != id && edge.dst != id);
            }

            self.label_edge_index.retain(|_, set| !set.is_empty());
        }
    }
}
