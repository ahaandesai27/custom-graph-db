use serde_json::Value;
use std::collections::HashMap;
use crate::properties::types::{property::{PropertyMap, PropertyValue}, property_query::{Cmp, PropertyQueryMap, PropertyQueryValue}};

pub fn decode_json(json: &str) -> PropertyMap {
    let raw: HashMap<String, Value> =
        serde_json::from_str(json).expect("Invalid JSON");

    let mut result: PropertyMap = HashMap::new();

    for (k, v) in raw {
        let pv = match v {
            Value::Bool(b) => PropertyValue::Bool(b),
            Value::String(s) => PropertyValue::Str(s),

            Value::Number(n) => {
                let i = n
                    .as_i64()
                    .expect("Non-integer number in JSON");

                PropertyValue::Int(i as i32)
            }

            Value::Array(_) | Value::Object(_) | Value::Null => {
                panic!("Unsupported value type for key '{}'", k);
            }
        };

        result.insert(k, pv);
    }

    result
}

fn parse_cmp(s: &str) -> Option<Cmp> {
    match s {
        "=" | "==" => Some(Cmp::Eq),
        "!=" => Some(Cmp::Neq),
        ">" => Some(Cmp::Gt),
        ">=" => Some(Cmp::Gte),
        "<" => Some(Cmp::Lt),
        "<=" => Some(Cmp::Lte),
        _ => None,
    }
}

pub fn decode_json_query(json: &str) -> PropertyQueryMap {
    let raw: HashMap<String, Value> =
        serde_json::from_str(json).expect("Invalid JSON");

    let mut result = HashMap::new();

    for (k, v) in raw {
        let qv = match v {
            Value::Bool(b) => PropertyQueryValue::Bool(b),

            Value::String(s) => PropertyQueryValue::Str(s),

            Value::Number(n) => {
                let i = n.as_i64().expect("Non-integer number");
                PropertyQueryValue::IntOp(i as i32, Cmp::Eq)
            }

            Value::Object(mut obj) => {
                let value = obj.remove("value").expect("Missing 'value'");
                let cmp = obj.remove("cmp").expect("Missing 'cmp'");

                let i = match value {
                    Value::Number(n) => n.as_i64().expect("Non-integer value"),
                    _ => panic!("'value' must be integer"),
                };

                let cmp = match cmp {
                    Value::String(s) => parse_cmp(&s).expect("Invalid cmp"),
                    _ => panic!("'cmp' must be string"),
                };

                PropertyQueryValue::IntOp(i as i32, cmp)
            }

            _ => panic!("Unsupported value type for key '{}'", k),
        };

        result.insert(k, qv);
    }

    result
}
