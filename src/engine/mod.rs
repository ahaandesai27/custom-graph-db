use crate::graph::Graph;
use crate::graph::node::Node;
use crate::parser::add_edge::parse::{AddEdgeStmt, parse_add_edge};
use crate::parser::create::parse::parse_create;
use crate::parser::select::parse::{SelectQuery, parse_select};
use crate::parser::query_parser::{QueryParser, Rule};
use crate::utils::shared::Shared;

use pest::Parser;

// pest::Parser is needed for QueryParser::parse

pub fn process_query(input: &str, graph: &mut Graph) -> Result<(), pest::error::Error<Rule>> {
    let mut pairs = QueryParser::parse(Rule::statement, input)?;
    let stmt = pairs.next().unwrap();
    match stmt.as_rule() {
        Rule::statement => {
            let inner = stmt.into_inner().next().unwrap(); // goes into the actual statement

            // matching first keyword, select, create etc
            match inner.as_rule() {
                Rule::create_stmt => {
                    let create_query = parse_create(inner);
                    let node_id = graph.add_node(&create_query.label, &create_query.properties);
                    let node_arc = graph.get_node(node_id).unwrap();
                    let guard = node_arc.read().unwrap();
                    // IMP: if i chain node_arc.guard call, the node_arc will be dropped and the guard will point to nothing
                    // Hence I must declare it separately 
                    println!("Node created: {}", guard);
                }
                Rule::select_stmt => {
                    let SelectQuery {
                        selected_labels,
                        pattern,
                        property_query,
                    } = parse_select(inner);

                    let nodes = graph.execute_pattern_chain(pattern);
                    let result: Vec<Shared<Node>> = nodes
                        .iter()
                        .cloned()
                        .filter(|node| {
                            let guard = node.read().unwrap();
                            selected_labels.contains(&guard.label)
                                && guard.is_satisfying_property(&property_query)
                        })
                        .collect();

                    for node in result {
                        let guard = node.read().unwrap();
                        println!("{}", guard);
                    }
                }
                Rule::add_edge_stmt => {
                    let AddEdgeStmt {
                        label, 
                        from,
                        to
                    } = parse_add_edge(inner);

                    let source_ids: Vec<_> = {
                        let nodes = graph
                            .get_nodes_satisfying_label_and_property(Some(&from.label), &from.filters);
                        nodes.into_iter().map(|n| {let guard = n.read().unwrap(); guard.id}).collect()
                    };

                    let source_count = source_ids.len();

                    let dest_ids: Vec<_> = {
                        let nodes = graph
                            .get_nodes_satisfying_label_and_property(Some(&to.label), &to.filters);
                        nodes.into_iter().map(|n| {let guard = n.read().unwrap(); guard.id}).collect()
                    };

                    let dest_count = dest_ids.len();

                    for src in &source_ids {
                        for dst in &dest_ids {
                            graph.add_edge(*src, *dst, &label);
                        }
                    }
                    
                    println!(
                        "Added {} '{}' edges ({} sources × {} destinations)",
                        source_count*dest_count, label, source_count, dest_count
                    );
                }
                _ => {
                    println!("This type of query does not exist!");
                    unreachable!()
                }
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

/*
unwrap extracts value inside Option or Result
next gets the next element from an iterator
into_inner goes into an inner type - useful for trees
 */
