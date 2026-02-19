// CRUD on graph edges 

use std::collections::HashSet;

use dashmap::DashSet;

use crate::{graph::{Graph, edge::{Edge, EdgeId}, node::NodeId}, utils::shared::{shared,Shared}};

impl Graph {
    pub fn add_edge(&self, src: NodeId, dst: NodeId, label: &str) -> EdgeId {
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
        let ids: HashSet<EdgeId> = self.label_edge_index
            .get(label)
            .map(|guard| guard.iter().map(|r| *r.key()).collect())
            // this line makes the guard on the dashset, and extracts its keys 
            .unwrap_or_default();
        
        ids.iter().filter_map(|id| self.get_edge(*id)).collect()
    }

    fn update_label_index(&self, edge_id: EdgeId, label: &str) {
        self.label_edge_index
            .entry(label.to_string())
            .or_insert_with(DashSet::new)
            .insert(edge_id);
    }

}