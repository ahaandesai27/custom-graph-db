// CRUD on graph nodes

use crate::graph::{
    Graph,
    node::{Node, properties::property_query_map::PropertyQueryMap},
};
use uuid::Uuid;
use std::collections::HashSet;


impl Graph {
    pub fn add_node(&mut self, label: &str) -> Uuid {
        // &mut self borrows the graph mutably and modifies it in place - thereby keeping ownership
        // &str is used so the function can accept any string input without forcing ownership or extra allocations

        let node = Node::new(label.to_string());
        let id = node.id;
        let label = node.label.clone();

        self.label_node_index
            .entry(label)
            .or_insert_with(Vec::new)
            .push(id);

        self.nodes.insert(node.id, node);

        id
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node> {
        // get always takes reference - should not take ownership
        // self is not being mutated, so borrow immutable reference
        // reads can occur concurrently -> threads will call this function
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: Uuid) -> Option<&mut Node> {
        // to get mutable references, if edits required
        // only one mutable reference at a time
        // only one mut -> only one write at a time
        self.nodes.get_mut(&id)
    }

    pub fn get_nodes_by_label(&self, label: &str) -> Vec<&Node> {
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

    pub fn out_neighbours(&self, node_id: Uuid) -> Vec<&Node> {
        self.out_index
            .get(&node_id)
            .into_iter()
            .flatten()
            .filter_map(|eid| self.edges.get(eid))
            .filter_map(|edge| self.nodes.get(&edge.dst))
            .collect()
    }

    pub fn in_neighbours(&self, node_id: Uuid) -> Vec<&Node> {
        self.in_index
            .get(&node_id)
            .into_iter()
            .flatten()
            .filter_map(|eid| self.edges.get(eid))
            .filter_map(|edge| self.nodes.get(&edge.src))
            .collect()
    }

    pub fn find_nodes_satisfying_label_and_property(
        &self,
        label: &str,
        properties: &PropertyQueryMap,
    ) -> Vec<&Node> {
        let ids: Vec<Uuid> = self
            .label_node_index
            .get(label)
            .into_iter()
            .flatten()
            .filter(|id| {
                self.get_node(**id)
                    .is_some_and(|node| node.is_satisfying_property(properties))
            })
            .copied()
            .collect();

        let result: Vec<&Node> = ids.iter().filter_map(|id| self.nodes.get(id)).collect();

        return result;
    }

    pub fn find_nodes_satisfying_property(&self, properties: PropertyQueryMap) -> Vec<&Node> {
        // needs to go to index hashmap
        let nodes: Vec<&Node> = self
            .nodes
            .values() // values gives us references to node
            .collect();

        let results: Vec<&Node> = nodes
            .into_iter()
            .filter(|node| node.is_satisfying_property(&properties))
            .collect();

        results

        // note: rust auto dereferences - BUT ONLY FOR METHOD CALLS
    }

    pub fn query_nodes_edges(&self, label_list: Vec<String>) -> Vec<&Node> {
        if label_list.is_empty() {
            return Vec::new();
        }

        let first_label = &label_list[0];
        let mut current_nodes: Vec<&Node> = self.get_nodes_by_label(first_label);

        let mut all_nodes: HashSet<Uuid> = current_nodes.iter().map(|n| n.id).collect();

        let mut i = 1;
        while i < label_list.len() {
            let edge_label = &label_list[i];
            let node_label = &label_list[i + 1];

            let mut next_nodes = Vec::new();
            let current_node_ids: Vec<Uuid> = current_nodes.iter().map(|n| n.id).collect();

            for node in &current_nodes {
                let edges = self.get_edges_by_label(edge_label);

                for edge in edges {
                    if edge.src == node.id && !current_node_ids.contains(&edge.dst) {
                        if let Some(dst_node) = self.get_node(edge.dst) {
                            if dst_node.label == *node_label {
                                all_nodes.insert(dst_node.id);
                                next_nodes.push(dst_node);
                            }
                        }
                    }
                }
            }

            current_nodes = next_nodes;
            i += 2;
        }

        all_nodes
            .iter()
            .filter_map(|id| self.get_node(*id))
            .collect()
    }
}
