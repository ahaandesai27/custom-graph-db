use crate::graph::Graph;
use crate::graph::node::Node;
use crate::parser::add_edge::parse::{AddEdgeStmt, parse_add_edge};
use crate::parser::create::parse::parse_create;
use crate::parser::select::parse::{SelectQuery, parse_select};
use crate::parser::query_parser::{QueryParser, Rule};
use crate::utils::shared::Shared;

use pest::Parser;

fn parse_statement<'a>(input: &'a str) -> Result<pest::iterators::Pair<'a, Rule>, pest::error::Error<Rule>> {
    let mut pairs = QueryParser::parse(Rule::statement, input)?;
    Ok(pairs.next().unwrap())
}

pub fn process_read_query(
    input: &str,
    graph: &Graph,
) -> Result<(), pest::error::Error<Rule>> {

    let stmt = parse_statement(input)?;
    let inner = stmt.into_inner().next().unwrap();

    match inner.as_rule() {
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
                // println!("{}", guard);
            }
        }

        _ => {
            println!("Write query rejected in read mode");
        }
    }

    Ok(())
}

pub fn process_write_query(
    input: &str,
    graph: &Graph,
) -> Result<(), pest::error::Error<Rule>> {

    let stmt = parse_statement(input)?;
    let inner = stmt.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::create_stmt => {
            let create_query = parse_create(inner);
            let node_id = graph.add_node(&create_query.label, &create_query.properties);
            // let node_arc = graph.get_node(node_id).unwrap();
            // let guard = node_arc.read().unwrap();
            // println!("Node created: {}", guard);
        }

        Rule::add_edge_stmt => {
            let AddEdgeStmt { label, from, to } = parse_add_edge(inner);

            let source_ids: Vec<_> = {
                let nodes = graph.get_nodes_satisfying_label_and_property(
                    Some(&from.label),
                    &from.filters
                );
                nodes.into_iter()
                    .map(|n| {
                        let guard = n.read().unwrap();
                        guard.id
                    })
                    .collect()
            };

            let dest_ids: Vec<_> = {
                let nodes = graph.get_nodes_satisfying_label_and_property(
                    Some(&to.label),
                    &to.filters
                );
                nodes.into_iter()
                    .map(|n| {
                        let guard = n.read().unwrap();
                        guard.id
                    })
                    .collect()
            };

            for src in &source_ids {
                for dst in &dest_ids {
                    graph.add_edge(*src, *dst, &label);
                }
            }

            // println!(
            //     "Added {} '{}' edges",
            //     source_ids.len() * dest_ids.len(),
            //     label
            // );
        }

        Rule::select_stmt => {
            println!("Read query rejected in write mode");
        }

        _ => unreachable!(),
    }

    Ok(())
}
