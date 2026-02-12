use std::collections::HashSet;

use crate::{
    graph::{Graph, node::{Node, NodeId}}, parser::select::parse::PatternElement,
};

impl Graph {
    fn traverse(
        &self,
        start_id: NodeId,
        edge_type: &str,
        min_hops: usize,
        max_hops: Option<usize>,
        target_label: &str,
    ) -> Vec<NodeId> {
        // BFS For edge traversal 
        let mut results = Vec::new();
        let mut visited: HashSet<NodeId> = HashSet::new();
        let mut frontier: Vec<(NodeId, usize)> = vec![(start_id, 0)];

        let max_depth = max_hops.unwrap_or(usize::MAX);

        while let Some((current_id, depth)) = frontier.pop() {
            if depth > max_depth {
                continue;
            }

            if depth >= min_hops && depth > 0 {
                if let Some(node) = self.get_node(current_id) {
                    if node.label == target_label {
                        results.push(current_id);
                    }
                }
            }

            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id);

            if depth == max_depth {
                continue;
            }

            let edges = self.get_edges_by_label(edge_type);

            for edge in edges {
                if edge.src == current_id {
                    frontier.push((edge.dst, depth + 1));
                }
            }
        }

        results
    }

    pub fn execute_pattern_chain(
        &self,
        pattern:Vec<PatternElement>,
    ) -> Vec<&Node> {

        if pattern.is_empty() {
            return Vec::new();
        }

        // First element must be a Node
        let first_label = match &pattern[0] {
            PatternElement::Node { label, .. } => {
                label.as_ref().expect("First node must have label")
            }
            _ => panic!("Pattern must start with Node"),
        };

        let mut current_nodes: Vec<&Node> =
            self.get_nodes_by_label(first_label);

        let mut all_nodes: HashSet<NodeId> =
            current_nodes.iter().map(|n| n.id).collect();

        let mut i = 1;

        while i < pattern.len() {
            // matches edge 
            let edge_pattern = match &pattern[i] {
                PatternElement::Edge {
                    edge_type,
                    min_hops,
                    max_hops,
                } => (edge_type, min_hops, max_hops),
                _ => panic!("Expected Edge at position {}", i),
            };

            // matches target node to it 
            let target_label = match &pattern[i + 1] {
                PatternElement::Node { label, .. } => {
                    label.as_ref().expect("Node must have label")
                }
                _ => panic!("Expected Node at position {}", i + 1),
            };

            let mut next_nodes: Vec<&Node> = Vec::new();

            // runs BFS depending on edge query 
            for node in &current_nodes {
                let reachable_ids = self.traverse(
                    node.id,
                    edge_pattern.0,
                    *edge_pattern.1,
                    *edge_pattern.2,
                    target_label,
                );

                for id in reachable_ids {
                    if let Some(n) = self.get_node(id) {
                        next_nodes.push(n);
                        all_nodes.insert(n.id);
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
