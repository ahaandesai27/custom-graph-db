use crate::graph::node::Node;
use crate::parser::query_parser::QueryParser;
use crate::utils::shared::Shared;
use crate::{graph::Graph, parser::query_parser::Rule};
use crate::parser::select::parse::{SelectQuery, parse_select};
use pest::Parser;

fn parse_statement<'a>(input: &'a str) -> Result<pest::iterators::Pair<'a, Rule>, pest::error::Error<Rule>> {
    let mut pairs = QueryParser::parse(Rule::statement, input)?;
    Ok(pairs.next().unwrap())
}

pub fn process_read_query(
    input: &str,
    graph: &Graph,
    log: bool,
) -> Result<(), pest::error::Error<Rule>> {

    let stmt = parse_statement(input)?;
    let inner = stmt.into_inner().next().unwrap();

    match inner.as_rule() {
        // match the select statement
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
            
            if log {
                for node in result {
                    let guard = node.read().unwrap();
                    println!("{}", guard);
                }
            }
        }

        _ => {
            println!("Write query rejected in read mode");
        }
    }

    Ok(())
}
