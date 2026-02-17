mod engine;
mod graph;
mod parser;
mod utils;

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

        r#"ADD EDGE E FROM (A PROPERTIES=(p3=true)) TO (B PROPERTIES=(p3=true))"#,
        r#"ADD EDGE F FROM (B PROPERTIES=(p3=true)) TO (C PROPERTIES=(p3=true))"#,

        r#"SELECT a,b,c FROM a:A-E->b:B-F->c:C"#
    ];


    for q in queries {
        process_query(q, &mut graph);
    }
}
