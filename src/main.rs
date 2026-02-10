#![allow(unused)]

mod engine;
mod graph;
mod parser;

use crate::engine::process_query;
use crate::graph::Graph;

fn main() {
    let mut graph: Graph = Graph::new();

    let queries = vec![
        r#"CREATE NODE LABEL=A PROPERTIES=(p1:2,  p2:"alpha",  p3:true)"#,
        r#"CREATE NODE LABEL=A PROPERTIES=(p1:8,  p2:"beta",   p3:false)"#,
        r#"CREATE NODE LABEL=A PROPERTIES=(p1:15, p2:"gamma",  p3:true)"#,

        r#"CREATE NODE LABEL=B PROPERTIES=(p1:1,  p2:"delta",  p3:true)"#,
        r#"CREATE NODE LABEL=B PROPERTIES=(p1:5,  p2:"echo",   p3:false)"#,
        r#"CREATE NODE LABEL=B PROPERTIES=(p1:9,  p2:"foxtrot",p3:true)"#,

        r#"CREATE NODE LABEL=C PROPERTIES=(p1:3,  p2:"hotel",  p3:false)"#,
        r#"CREATE NODE LABEL=C PROPERTIES=(p1:7,  p2:"india",  p3:true)"#,
        r#"CREATE NODE LABEL=C PROPERTIES=(p1:11, p2:"juliet", p3:true)"#,
    ];



    for q in queries {
        process_query(q, &mut graph);
    }

    let ids_a: Vec<_> = graph.label_node_index["A"].iter().copied().collect();        // iter creates a borrow
    let ids_b: Vec<_> = graph.label_node_index["B"].iter().copied().collect();
    let ids_c: Vec<_> = graph.label_node_index["C"].iter().copied().collect();

    for &node1 in &ids_a {
        for &node2 in &ids_b {
            graph.add_edge(node1, node2, "E");
        }
    }

    for &node1 in &ids_b {
        for &node2 in &ids_c {
            graph.add_edge(node1, node2, "F");
        }
    }

    let input = r#"SELECT a,b,c FROM a:A-E->b:B-F->c:C WHERE (p1>=5, p3=true)"#;
    // later , can also add a.p1>=5 (so it looks only in a for p1>=5)
    process_query(input, &mut graph);
}
