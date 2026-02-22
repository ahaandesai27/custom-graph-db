use crate::graph::node::Node;
use crate::parser::query_parser::QueryParser;
use crate::parser::select::parse::{SelectQuery, parse_select};
use crate::utils::shared::Shared;
use crate::{graph::Graph, parser::query_parser::Rule};
use pest::Parser;

fn parse_statement<'a>(
    input: &'a str,
) -> Result<pest::iterators::Pair<'a, Rule>, pest::error::Error<Rule>> {
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

            let rows = graph.execute_pattern_chain(pattern);

            let result: Vec<Vec<Shared<Node>>> = rows
                .into_iter()
                .filter(|row| {
                    // only keeps rows where all nodes matches the property 
                    row.iter().all(|node| {
                        let guard = node.read().unwrap();
                        selected_labels.contains(&guard.label)
                            && guard.is_satisfying_property(&property_query)
                    })
                })
                .collect();
            
            if log {
                for (i, row) in result.iter().enumerate() {
                    println!("Row {}:", i + 1);

                    for node in row {
                        let guard = node.read().unwrap();
                        println!("  {} {:?}", guard.label, guard.property_map);
                    }

                    println!();
                }
            }
        }

        _ => {
            println!("Write query rejected in read mode");
        }
    }

    Ok(())
}
