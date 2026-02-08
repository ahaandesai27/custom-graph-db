

use std::collections::HashMap;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::graph::node;
use crate::graph::node::properties::property_query_map::PropertyQueryMap;
use crate::parser::query_parser::Rule;

pub struct SelectQuery {
    pub variables: Vec<String>,
    pub node_labels: HashMap<String, String>,
    pub node_edges: Vec<String>,
}

pub fn parse_select(pair: Pair<Rule>) -> SelectQuery {
    // going after "select_stmt"
    let mut inner = pair.into_inner();
    inner.next();           // SELECT 

    // Label list of nodes to fetch 
    let var_clause = inner.next().unwrap();
    let variables_pair = var_clause.into_inner();
    let mut variables: Vec<String> = Vec::new();

    for v in variables_pair {
        variables.push(v.as_str().to_string());
    }
    // FROM
    inner.next();

    // Relation query 
    let relation_clause = inner.next().unwrap();
    let relations = relation_clause.into_inner();

    let mut node_edges: Vec<String> = Vec::new();
    let mut node_labels: HashMap<String, String> = HashMap::new();

    for v in relations {
        let s = v.as_str();
        if let Some((left, right)) = s.split_once(':') {
            node_edges.push(right.to_string());

            node_labels.insert(left.to_string(), right.to_string());
        } else {
            node_edges.push(s.to_string());
        }
    }


    SelectQuery { variables, node_labels, node_edges }
}
