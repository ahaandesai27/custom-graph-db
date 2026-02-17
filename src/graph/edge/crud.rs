// CRUD on graph edges 

use crate::graph::{Graph, edge::{Edge, EdgeId}, node::NodeId};

impl Graph {
    pub fn add_edge(&mut self, src: NodeId, dst: NodeId, label: &str) -> EdgeId {
        let edge = Edge::new(self.edge_idgen.next_id(), src, dst, label.to_string());
        let edge_id = edge.id;
        self.edges.insert(edge_id, edge);
        self.update_label_index(edge_id, label);
        edge_id
    }
    
    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
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

    fn update_label_index(&mut self, edge_id: EdgeId, label: &str) {
        self.label_edge_index
            .entry(label.to_string())
            .or_insert_with(Vec::new)
            .push(edge_id);
    }

}