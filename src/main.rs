#![allow(unused)]

mod graph;
mod properties;
mod interpreter;

use crate::{graph::graph::Graph, properties::{decode_json::decode_json_query, types::property_query::PropertyQueryMap}};
use crate::interpreter::test::process_query;

fn test_graph() {
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
    let node4 = graph.get_node_mut(node4_id).unwrap();
    let json = r#"
{
"name": "NodeA",
"label": "Person",
"age": 25,
"active": true
}
"#;
    node4.add_properties_from_json(json);
    println!("{}", node4);


    let property_query_json = r#"
{
  "age": {
    "value": 20,
    "cmp": ">"
  }
}
"#;
    let map: PropertyQueryMap = decode_json_query(property_query_json);
    
    let result = node4.is_satisfying_property(&map);
    println!("{}", result);
}

fn main() {
    let input = r#"CREATE NODE Person (name:"Alice", age:25, female:true)"#;
    let mut graph = Graph::new();
    process_query(input, &mut graph);
}
