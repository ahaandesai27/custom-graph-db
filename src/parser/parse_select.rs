

use std::collections::{HashMap, HashSet};

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::graph::node;
use crate::graph::node::properties::property_query_map::{Cmp, PropertyQueryMap, PropertyQueryValue};
use crate::parser::query_parser::Rule;

fn parse_cmp(op: &str) -> Cmp {
    match op {
        "="  => Cmp::Eq,
        "!=" => Cmp::Neq,
        ">"  => Cmp::Gt,
        ">=" => Cmp::Gte,
        "<"  => Cmp::Lt,
        "<=" => Cmp::Lte,
        _ => unreachable!(),
    }
}

fn parse_value(raw: &str, cmp: Cmp) -> PropertyQueryValue {
    if raw == "true" || raw == "false" {
        return PropertyQueryValue::Bool(raw == "true");
    }

    if let Ok(num) = raw.parse::<i32>() {
        return PropertyQueryValue::IntOp(num, cmp);
    }

    PropertyQueryValue::Str(raw.to_string())
}

pub struct SelectQuery {
    pub variables: Vec<String>,
    pub selected_labels: HashSet<String>,  
    pub node_edges: Vec<String>,
    pub property_query: PropertyQueryMap
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

    // Building selected labels
    let mut selected_labels = HashSet::new();
    for var in &variables {
        if let Some(label) = node_labels.get(var) {
            selected_labels.insert(label.clone());
        }
    }

    // Properties 
    let mut property_query: PropertyQueryMap = HashMap::new();

    if let Some(where_pair) = inner.next() {
        for cmp_pair in where_pair.into_inner() {
            if cmp_pair.as_rule() != Rule::comparison { continue; }

            let mut cmp_inner = cmp_pair.into_inner();

            let key = cmp_inner.next().unwrap().as_str().to_string();
            let op_str = cmp_inner.next().unwrap().as_str();
            let value_raw = cmp_inner.next().unwrap().as_str();

            let cmp = parse_cmp(op_str);
            let value = parse_value(value_raw, cmp.clone());

            property_query.insert(key, value);
        }
    }


    SelectQuery { variables, selected_labels, node_edges, property_query }
}
