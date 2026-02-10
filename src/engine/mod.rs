use std::collections::HashSet;

use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph::node::properties::property_query_map::PropertyQueryMap;
use crate::parser::parse_create::parse_create;
use crate::parser::parse_select::{SelectQuery, parse_select};
use crate::parser::query_parser::{QueryParser, Rule};

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
                    let node_id = graph.add_node(&create_query.label);

                    let node: &mut Node = graph.get_node_mut(node_id).unwrap();
                    node.set_properties(create_query.properties);

                    println!("Node created: {}", node);
                }
                Rule::select_stmt => {
                    let SelectQuery {
                        selected_labels,
                        node_edges,
                        property_query,
                        ..
                    } = parse_select(inner);

                    let nodes = graph.query_nodes_edges(node_edges);

                    let result: Vec<&Node> = nodes
                        .iter()
                        .copied()
                        .filter(|node| {
                            selected_labels.contains(&node.label)
                                && node.is_satisfying_property(&property_query)
                        })
                        .collect();

                    for node in result {
                        println!("{}", node);
                    }
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
