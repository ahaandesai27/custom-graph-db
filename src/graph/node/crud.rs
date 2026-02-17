// CRUD on graph nodes

use crate::graph::{
    Graph,
    node::{Node, NodeId, properties::{property_map::PropertyMap, property_query_map::PropertyQueryMap}},
};


impl Graph {
    pub fn add_node(&mut self, label: &str, properties: &PropertyMap) -> NodeId {
        // &mut self borrows the graph mutably and modifies it in place - thereby keeping ownership
        // &str is used so the function can accept any string input without forcing ownership or extra allocations

        let node = Node::new(self.node_idgen.next_id(), label.to_string(), properties.clone());
        let id = node.id;
        let label = node.label.clone();

        self.label_node_index
            .entry(label)
            .or_insert_with(Vec::new)
            .push(id);

        self.nodes.insert(node.id, node);

        id
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        // get always takes reference - should not take ownership
        // self is not being mutated, so borrow immutable reference
        // reads can occur concurrently -> threads will call this function
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
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

    pub fn find_nodes_satisfying_label_and_property(
        &self,
        label: &str,
        properties: &PropertyQueryMap,
    ) -> Vec<&Node> {
        let ids: Vec<NodeId> = self
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
}
