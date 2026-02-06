#![allow(unused)]

mod graph;
mod parser;
mod engine;

use crate::{graph::Graph};
use crate::engine::process_query;

fn main() {
    let input = r#"CREATE NODE LABEL=Person PROPERTIES=(name:"Alice", age:25, female:true)"#;
    let mut graph = Graph::new();
    process_query(input, &mut graph);
}
