use crate::graph::node::Node;
use crate::propertymap::decode_json::decode_json;

impl Node {
    pub fn add_properties_from_json(&mut self, json: &str) {
        // Assuming added_properties is not needed after 
        
        let added_properties = decode_json(json);
        for (key, value) in added_properties {
            self.property_map.insert(key, value);
        }
    }
}