use crate::graph::node::Node;
use crate::properties::decode_json::decode_json;
use crate::properties::types::property::{PropertyMap, PropertyValue};
use crate::properties::types::property_query::{Cmp, PropertyQueryMap, PropertyQueryValue};

impl Node {
    pub fn set_properties(&mut self, properties: PropertyMap) {
        self.property_map = properties;
    }
    
    pub fn add_properties_from_json(&mut self, json: &str) {
        // Assuming added_properties is not needed after

        let added_properties: PropertyMap = decode_json(json);
        for (key, value) in added_properties {
            self.property_map.insert(key, value);
        }
    }

    pub fn is_satisfying_property(&self, query: &PropertyQueryMap) -> bool {
        for (k, qv) in query {
            let pv = match self.property_map.get(k) {
                Some(v) => v,
                None => return false,
            };

            let matches = match (qv, pv) {
                (PropertyQueryValue::Bool(b1), PropertyValue::Bool(b2)) => b1 == b2,

                (PropertyQueryValue::Str(s1), PropertyValue::Str(s2)) => s1 == s2,

                (PropertyQueryValue::IntOp(qv, cmp), PropertyValue::Int(pv)) => match cmp {
                    Cmp::Eq => pv == qv,
                    Cmp::Neq => pv != qv,
                    Cmp::Gt => pv > qv,
                    Cmp::Gte => pv >= qv,
                    Cmp::Lt => pv < qv,
                    Cmp::Lte => pv <= qv,
                },

                _ => false, // type mismatch
            };

            if !matches {
                return false;
            }
        }

        true
    }
}
