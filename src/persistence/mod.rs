pub mod edges;
pub mod nodes;
pub struct Store {
    node_data: String,
    edge_data: String, 
}

impl Store {
    pub fn new(node_data: String, edge_data: String) -> Self {
        Store { node_data, edge_data }
    }
}
