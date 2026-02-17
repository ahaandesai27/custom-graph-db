// CRUD on graph nodes

use std::collections::HashSet;

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
    pub fn add_node(&mut self, label: &str, properties: &PropertyMap) -> NodeId {
        // &mut self borrows the graph mutably and modifies it in place - thereby keeping ownership
        // &str is used so the function can accept any string input without forcing ownership or extra allocations

        let id = self.node_idgen.next_id();
        let node = shared(Node::new(id, label.to_string(), properties.clone()));

        self.label_node_index
            .entry(label.to_string())
            .or_insert_with(HashSet::new)
            .insert(id);

        self.nodes.insert(id, node);

        id
    }

    pub fn get_node(&self, id: NodeId) -> Option<Shared<Node>> {
        self.nodes.get(&id).map(|entry| entry.clone())

        // it returns a guard
        // we must clone the arc  - otherwise the reference goes out of scope
        // else the guard to the node dies after we leave scope

        // if lock is needed, lock the node on return (in the function that calls) and update
    }

    pub fn get_nodes_by_label(&self, label: &str) -> Vec<Shared<Node>> {
        // rust cannot prove two &Nodes are different
        // hence returning two &mut Nodes may cause an error
        // only immutable returns for this one
        self.label_node_index
            .get(label)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_node(*id))
            .collect::<Vec<_>>()
    }

    pub fn get_node_ids_by_label(&self, label: &str) -> Option<&HashSet<NodeId>> {
        // rust cannot prove two &Nodes are different
        // hence returning two &mut Nodes may cause an error
        // only immutable returns for this one

        // need to make concurrent later
        self.label_node_index
            .get(label)
    }

    pub fn get_nodes_satisfying_label_and_property(
        &self,
        label: Option<&str>,
        properties: &PropertyQueryMap,
    ) -> Vec<Shared<Node>> {
        match label {
            Some(l) => self
                .label_node_index
                .get(l)
                .into_iter()
                .flatten()
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
                .collect(),
            None => self
                .nodes
                .iter()
                .filter_map(|entry| {
                    let node = entry.value();
                    let guard = node.read().unwrap();
                    if guard.is_satisfying_property(properties) {
                        Some(node.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
