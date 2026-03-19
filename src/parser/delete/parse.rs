use pest::iterators::Pair;

use crate::{graph::node::NodeId, parser::query_parser::Rule};

pub struct DeleteStmt {
    pub id: NodeId
}

pub fn parse_delete(pair: Pair<Rule>) -> DeleteStmt {
    let mut inner = pair.into_inner();
    
    for _ in 0..3 {
        // only DELETE, NODE, WHERE appear as tokens, rest are skipped
        inner.next();
    }

    let id: NodeId = inner.next().unwrap().as_str().parse::<NodeId>().unwrap();
    DeleteStmt { id }
}
