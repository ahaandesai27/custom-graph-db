use crate::{
    graph::{Graph, node::{Node, NodeId}},
    parser::select::parse::PatternElement,
    utils::shared::Shared,
};

impl Graph {

    fn traverse(
        &self,
        start_id: NodeId,
        edge_type: &str,
        min_hops: usize,
        max_hops: Option<usize>,
        target_label: &str,
    ) -> Vec<Vec<NodeId>> {

        let mut results = Vec::new();

        let mut frontier: Vec<(NodeId, usize, Vec<NodeId>)> =
            vec![(start_id, 0, vec![start_id])];

        let max_depth = max_hops.unwrap_or(usize::MAX);

        while let Some((current_id, depth, path)) = frontier.pop() {

            if depth > max_depth {
                continue;
            }

            if depth >= min_hops && depth > 0 {
                if let Some(node) = self.get_node(current_id) {
                    let guard = node.read().unwrap();
                    if guard.label == target_label {
                        results.push(path.clone());
                    }
                }
            }

            if depth == max_depth {
                continue;
            }

            let edges = self.get_edges_by_label(edge_type);

            for edge in edges {
                if edge.src == current_id {

                    let mut new_path = path.clone();
                    new_path.push(edge.dst);

                    frontier.push((edge.dst, depth + 1, new_path));
                }
            }
        }

        results
    }

    pub fn execute_pattern_chain(
        &self,
        pattern: Vec<PatternElement>,
    ) -> Vec<Vec<Shared<Node>>> {

        if pattern.is_empty() {
            return Vec::new();
        }

        let first_label = match &pattern[0] {
            PatternElement::Node { label, .. } => {
                label.as_ref().expect("First node must have label")
            }
            _ => panic!("Pattern must start with Node"),
        };

        let start_ids = self
            .get_node_ids_by_label(first_label)
            .unwrap()
            .clone();

        let mut rows: Vec<Vec<NodeId>> =
            start_ids.into_iter().map(|id| vec![id]).collect();

        let mut i = 1;

        while i < pattern.len() {

            let (edge_type, min_hops, max_hops) = match &pattern[i] {
                PatternElement::Edge {
                    edge_type,
                    min_hops,
                    max_hops,
                } => (edge_type, min_hops, max_hops),
                _ => panic!("Expected Edge at position {}", i),
            };

            let target_label = match &pattern[i + 1] {
                PatternElement::Node { label, .. } => {
                    label.as_ref().expect("Node must have label")
                }
                _ => panic!("Expected Node at position {}", i + 1),
            };

            let mut new_rows = Vec::new();

            for row in rows {

                let last_id = *row.last().unwrap();

                let paths = self.traverse(
                    last_id,
                    edge_type,
                    *min_hops,
                    *max_hops,
                    target_label,
                );

                for path in paths {

                    let mut extended = row.clone();

                    for id in path.into_iter().skip(1) {
                        extended.push(id);
                    }

                    new_rows.push(extended);
                }
            }

            rows = new_rows;
            i += 2;
        }

        rows.into_iter()
            .map(|row| {
                row.into_iter()
                    .filter_map(|id| self.get_node(id))
                    .collect()
            })
            .collect()
    }
}