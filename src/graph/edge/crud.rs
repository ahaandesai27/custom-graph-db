use std::{sync::Arc};
use dashmap::DashSet;
use crate::{graph::{Graph, edge::{Edge}, node::NodeId}};

impl Graph {
    pub fn add_edge(&self, src: NodeId, dst: NodeId, label: &str) {
        let edge = Arc::new(Edge::new(src, dst, label.to_string()));
        let cloned_arc = edge.clone();

        self.edges.insert(edge);
        self.update_label_index(cloned_arc, label);

        self.store.write_edge(src, dst, label);
    }

    pub fn get_edges_by_label(&self, label: &str) -> Vec<Arc<Edge>> {
        if let Some(set_ref) = self.label_edge_index.get(label) {
            set_ref.iter().map(|e| e.clone()).collect()
        } else {
            Vec::new()
        }
    }

    fn update_label_index(&self, edge: Arc<Edge>, label: &str) {
        self.label_edge_index
            .entry(label.to_string())
            .or_insert_with(DashSet::new)
            .insert(edge);
    }

}