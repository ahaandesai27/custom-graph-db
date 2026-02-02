use serde_json::Value;
use std::collections::HashMap;

pub fn decode_json(json: &str) -> HashMap<String, Value> {
    let property_map: HashMap<String, Value> = serde_json::from_str::<HashMap<String, Value>>(json).unwrap();
    return property_map;
}   