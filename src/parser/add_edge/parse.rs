use pest::iterators::Pair;
use std::collections::HashMap;
use crate::{graph::node::properties::property_query_map::{Cmp, PropertyQueryMap, PropertyQueryValue}, parser::query_parser::Rule};


pub struct AddEdgeStmt {
    pub label: String,
    pub from: NodeClause,
    pub to: NodeClause,
}

pub struct NodeClause {
    pub label: String,
    pub filters: PropertyQueryMap,
}


pub fn parse_add_edge(pair: Pair<Rule>) -> AddEdgeStmt {
    let mut inner = pair.into_inner();

    inner.next();       // ADD 
    inner.next();       // EDGE 

    let label = inner.next().unwrap().as_str().to_string();

    inner.next();       // FROM 
    let from = parse_node_clause(inner.next().unwrap());

    inner.next();       // TO 
    let to = parse_node_clause(inner.next().unwrap());

    AddEdgeStmt { label, from, to }
}

fn parse_node_clause(pair: Pair<Rule>) -> NodeClause {
    let mut inner = pair.into_inner();
    let label = inner.next().unwrap().as_str().to_string();

    let mut filter_rules = inner.next().unwrap();
    let filters = parse_property_clause(filter_rules);    
    NodeClause { label, filters }
}

fn parse_property_clause(pair: Pair<Rule>) -> PropertyQueryMap {
    let mut inner = pair.into_inner();

    inner.next();              // PROPERTIES 

    let map_pair = inner.next().unwrap();

    map_pair
        .into_inner()
        .map(parse_comparison)
        .collect()
}

fn parse_comparison(pair: Pair<Rule>) -> (String, PropertyQueryValue) {
    let mut inner = pair.into_inner();

    let field = inner.next().unwrap().as_str().to_string();
    let cmp = parse_cmp(inner.next().unwrap());

    let value_pair = inner.next().unwrap();
    let literal = value_pair.into_inner().next().unwrap();

    let value = match literal.as_rule() {
        Rule::string => {
            let s = literal.as_str();
            PropertyQueryValue::Str(s[1..s.len() - 1].to_string())
        }
        Rule::number => {
            let num: i32 = literal.as_str().parse().unwrap();
            PropertyQueryValue::IntOp(num, cmp)
        }
        Rule::boolean => {
            let b = literal.as_str() == "true";
            PropertyQueryValue::Bool(b)
        }
        _ => unreachable!(),
    };

    (field, value)
}

fn parse_cmp(pair: Pair<Rule>) -> Cmp {
    match pair.as_str() {
        "="  => Cmp::Eq,
        "!=" => Cmp::Neq,
        ">"  => Cmp::Gt,
        "<"  => Cmp::Lt,
        ">=" => Cmp::Gte,
        "<=" => Cmp::Lte,
        _ => unreachable!(),
    }
}
