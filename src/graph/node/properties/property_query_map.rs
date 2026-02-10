use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Cmp {
    Eq, 
    Neq,
    Gt, 
    Gte, 
    Lt,
    Lte,
}

pub enum PropertyQueryValue {
    Str(String),                // equality only
    Bool(bool),                 // equality only
    IntOp(i32, Cmp),            // operators 
}

pub type PropertyQueryMap = HashMap<String, PropertyQueryValue>;


/*
what will they look like
{
    boolean=true,
    string="example",
    int=5 (by default equal) / int  = {
        value = 5,
        cmp = ">="
    }
} 
 */