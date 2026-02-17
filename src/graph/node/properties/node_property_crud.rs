use crate::graph::node::Node;
use crate::graph::node::properties::property_map::{PropertyValue};
use crate::graph::node::properties::property_query_map::{Cmp, PropertyQueryMap, PropertyQueryValue};

impl Node {
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
