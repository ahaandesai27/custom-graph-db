use std::collections::HashMap;

#[derive(serde::Serialize)]
#[serde(untagged)]                      // with this, even the type(Str, Int, Bool) is added in the serialisation
pub enum PropertyValue {
    Bool(bool),
    Str(String),
    Int(i32)
}

pub type PropertyMap = HashMap<String, PropertyValue>;

