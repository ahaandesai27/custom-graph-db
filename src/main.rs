#![allow(unused)]



use crate::engine::read::process_read_query;
use crate::engine::write::process_write_query;
use crate::graph::Graph;
use std::sync::{Arc};
use std::thread;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod engine;
mod graph;
mod parser;
mod utils;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let graph = Arc::new(Graph::new());
    println!("Server listening on port 8000...");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let graph_ref = Arc::clone(&graph);

        tokio::spawn(async move {
            let mut buf = [0; 1024];    // 1024 size buffer

            loop {
                let n  = socket.read(&mut buf).await.unwrap(); // read data 
                if n == 0 {break;}
                let query = String::from_utf8_lossy(&buf[..n]);
                if query.starts_with("SELECT") {
                    let result = process_read_query(&query, &graph_ref, true);
                    
                    // send back results 
                    let response = match result {
                        Ok(result) => result.join("\n"),
                        Err(e) => format!("Error: {}", e),
                    };
                    socket.write_all(response.as_bytes()).await.unwrap();
                    socket.write_all(b"\n").await.unwrap();
                } else {
                    let result = process_write_query(&query, &graph_ref, true);
                    let response = match result {
                        Ok(msg) => msg,
                        Err(e) => format!("Error: {}", e),
                    };
                    socket.write_all(response.as_bytes()).await.unwrap();
                    socket.write_all(b"\n").await.unwrap();
                }
            }

        });
    }
}


// const QUERIES: &[&str] = &[
//     r#"CREATE NODE LABEL=A PROPERTIES=(p1:2, p2:"alpha", p3:true)"#,
//     r#"CREATE NODE LABEL=B PROPERTIES=(p1:5, p2:"beta", p3:false)"#,
//     r#"CREATE NODE LABEL=C PROPERTIES=(p1:7, p2:"gamma", p3:true)"#,
//     r#"ADD EDGE E FROM (A PROPERTIES=(p3=true)) TO (B PROPERTIES=(p3=false))"#,
//     r#"DELETE NODE WHERE ID=1"#,
//     r#"DELETE NODE WHERE ID=2"#,
//     // r#"SELECT a FROM a:A"#,
//     // r#"SELECT b FROM b:B"#,
//     // r#"SELECT c FROM c:C"#,
//     // r#"SELECT a,b FROM a:A-E->b:B"#
// ];

// fn run_sequential_then_concurrent(iterations: usize, thread_count: usize) {
//     // Sequential: equivalent work to n threads * iterations
//     let graph_seq = Arc::new(Graph::new());
//     for _ in 0..(thread_count * iterations) {
//         run_queries(&graph_seq);
//     }
//     let seq_nodes = graph_seq.node_count();
//     let seq_edges = graph_seq.edge_count();
//     println!("Sequential: {} nodes, {} edges", seq_nodes, seq_edges);

//     // Concurrent: n threads each doing iterations cycles
//     let graph_conc = Arc::new(Graph::new());
//     let handles: Vec<_> = (0..thread_count)
//         .map(|_| {
//             let g = Arc::clone(&graph_conc);
//             thread::spawn(move || {
//                 for _ in 0..iterations {
//                     run_queries(&g);
//                 }
//             })
//         })
//         .collect();

//     for h in handles { h.join().unwrap(); }

//     let conc_nodes = graph_conc.node_count();
//     let conc_edges = graph_conc.edge_count();
//     println!("Concurrent: {} nodes, {} edges", conc_nodes, conc_edges);

//     // node count should match exactly - creates are unconditional
//     assert_eq!(seq_nodes, conc_nodes, "node count mismatch");
//     // edges wont match exactly due to ordering but should be in same ballpark
//     println!("Edge delta: {}", (seq_edges as i64 - conc_edges as i64).abs());
// }

// fn run_queries(graph: &Graph) {
//     for q in QUERIES {
//         if q.starts_with("SELECT") {
//             process_read_query(q, graph, false).unwrap();
//         } else {
//             process_write_query(q, graph, false).unwrap();
//         }
//     }
// }

// fn main() {
//     run_sequential_then_concurrent(5, 10);
//     // let graph = Graph::new();
//     // run_queries(&graph);
// }
