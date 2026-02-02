mod graph;
mod propertymap;

use crate::graph::{graph::Graph};
use std::collections::HashMap;

fn try_serde() {
    let json = r#"{"a":"1","b":"hello","c":"true"}"#;       // raw string lit in rust 
    let map: HashMap<String, String> = serde_json::from_str(json).unwrap();
    println!("{} {} {}", map["a"], map["b"], map["c"]);
}

fn main() {
    let mut graph: Graph = Graph::new();
    let node1_id = graph.add_node("ABC");
    let node2_id = graph.add_node("DEF");
    let node3_id = graph.add_node("XYZ");
    
    graph.add_edge(node1_id, node2_id, "EDGE_1");
    graph.add_edge(node1_id, node3_id, "EDGE_2");

    let neighbours1 = graph.out_neighbours(node1_id);

    for node in neighbours1 {
        println!("{}", node);
    }

    let node4_id = graph.add_node("TEST");
    let Node4 = graph.get_node_mut(node4_id).unwrap();
    let json = r#"
{
"name": "NodeA",
"label": "Person",
"age": 25,
"active": true
}
"#;
    Node4.add_properties_from_json(json);
    println!("{}", Node4);
}
