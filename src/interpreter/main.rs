use crate::graph::node::Node;
use crate::interpreter::parse_create::parse_create;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use crate::graph::graph::Graph;

#[derive(Parser)]
#[grammar = "src/interpreter/query.pest"]
struct QueryParser;

pub fn process_query(input: &str, graph: &mut Graph) -> Result<(), pest::error::Error<Rule>> {
    let mut pairs = QueryParser::parse(Rule::statement, input)?;
    let stmt = pairs.next().unwrap();

    match stmt.as_rule() {
        Rule::statement => {
            let inner = stmt.into_inner().next().unwrap();      // goes into the actual statement

            // matching first keyword, select, create etc
            match inner.as_rule() {
                Rule::create_stmt => {
                    let create_node = parse_create(inner);
                    let node_id = graph.add_node(&create_node.label);

                    let node: &mut Node = graph.get_node_mut(node_id).unwrap();
                    node.set_properties(create_node.properties);

                    println!("Node created: {}", node);
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