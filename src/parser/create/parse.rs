
use std::collections::HashMap;
use pest::iterators::Pair;
use crate::{graph::node::properties::property_map::{PropertyMap, PropertyValue}, parser::query_parser::Rule};

pub struct CreateNode {
    pub label: String,
    pub properties: PropertyMap,
}


fn parse_value(pair: Pair<Rule>) -> PropertyValue {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::string => {
            PropertyValue::Str(inner.as_str().trim_matches('"').to_string())
        }
        Rule::number => {
            PropertyValue::Int(inner.as_str().parse().unwrap())
        }
        Rule::boolean => {
            PropertyValue::Bool(if inner.as_str() == "true" {true} else {false})
        }
        _ => unreachable!(),
    }
}


pub fn parse_create(pair: Pair<Rule>) -> CreateNode {
    let mut inner = pair.into_inner();
    inner.next(); // CREATE
    inner.next(); // NODE


    let label_clause = inner.next().unwrap();   
    let mut label_inner = label_clause.into_inner();
    label_inner.next();         // LABEL 
    let label = label_inner.next().unwrap().as_str().to_string(); // getting the label 

    let mut properties = HashMap::new();

    if let Some(props_clause) = inner.next() {
        // props_clause contains a properties rule

        let mut inner = props_clause.into_inner();
        inner.next();
        let props = inner.next().unwrap();

        for prop in props.into_inner() {
            let mut p = prop.into_inner();
            let key = p.next().unwrap().as_str().to_string();
            let val = parse_value(p.next().unwrap());
            properties.insert(key, val);
        }
    }

    CreateNode { label, properties }
}
