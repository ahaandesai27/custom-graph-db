// CRUD on graph edges 

use crate::graph::graph::Graph;
use crate::graph::node::Node;
use uuid::Uuid;

impl Graph {
    pub fn add_node(&mut self, label: &str) -> Uuid {
        // &mut self borrows the graph mutably and modifies it in place - thereby keeping ownership 
        // &str is used so the function can accept any string input without forcing ownership or extra allocations

        let node = Node::new(label.to_string());
        let id = node.id;
        let label = node.label.clone();

        self.label_node_index.
            entry(label)
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

    pub fn get_node_by_label(&self, label: &str) -> Vec<&Node> {
        // rust cannot prove two &Nodes are different
        // hence returning two &mut Nodes may cause an error 
        // only immutable returns for this one 
        self.nodes
            .values()
            .filter(|n| n.label == label)   // this is a rust lambda 
            .collect()
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
}
