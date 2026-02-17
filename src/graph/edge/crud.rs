// CRUD on graph edges 

use std::collections::HashSet;

use crate::{graph::{Graph, edge::{Edge, EdgeId}, node::NodeId}, utils::shared::{shared,Shared}};

impl Graph {
    pub fn add_edge(&mut self, src: NodeId, dst: NodeId, label: &str) -> EdgeId {
        let edge_id = self.edge_idgen.next_id();
        let edge = shared(Edge::new(edge_id, src, dst, label.to_string()));
        self.edges.insert(edge_id, edge);
        self.update_label_index(edge_id, label);
        edge_id
    }
    
    pub fn get_edge(&self, id: EdgeId) -> Option<Shared<Edge>> {
        self.edges.get(&id).map(|entry| entry.clone())
    }

    pub fn get_edges_by_label(&self, label: &str) -> Vec<Shared<Edge>> {
        self.label_edge_index
            .get(label)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_edge(*id))
            .collect::<Vec<_>>()
    }

    fn update_label_index(&mut self, edge_id: EdgeId, label: &str) {
        self.label_edge_index
            .entry(label.to_string())
            .or_insert_with(HashSet::new)
            .insert(edge_id);
    }

}