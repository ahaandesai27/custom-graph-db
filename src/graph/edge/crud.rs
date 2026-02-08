// CRUD on graph edges 

use crate::graph::{Graph, edge::Edge};
use uuid::Uuid;

impl Graph {
    pub fn add_edge(&mut self, src: Uuid, dst: Uuid, label: &str) -> Uuid {
        let edge = Edge::new(src, dst, label.to_string());
        let edge_id = edge.id;
        self.edges.insert(edge_id, edge);
        self.update_label_index(edge_id, label);
        self.update_out_index(edge_id, src);
        self.update_in_index(edge_id, dst);

        edge_id
    }
    
    pub fn get_edge(&self, id: Uuid) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn get_edges_by_label(&self, label: &str) -> Vec<&Edge> {
        self.label_edge_index
            .get(label)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_edge(*id))
            .collect::<Vec<_>>()
    }

    fn update_label_index(&mut self, edge_id: Uuid, label: &str) {
        self.label_edge_index
            .entry(label.to_string())
            .or_insert_with(Vec::new)
            .push(edge_id);
    }

    fn update_out_index(&mut self, edge_id: Uuid, src: Uuid) {
        self.out_index
            .entry(src)
            .or_insert_with(Vec::new)
            .push(edge_id);
    }

    fn update_in_index(&mut self, edge_id: Uuid, dst: Uuid) {
        self.in_index
            .entry(dst)
            .or_insert_with(Vec::new)
            .push(edge_id);
    }
}