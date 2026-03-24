pub mod edge;
pub mod idgen;
pub mod node;
pub mod traverse;

use std::sync::Arc;


use dashmap::{DashMap, DashSet};

use crate::graph::node::properties::property_map::PropertyMap;
use crate::persistence::Store;
use crate::utils::shared::shared;
use crate::{
    graph::{
        edge::Edge,
        idgen::IdGenerator,
        node::{Node, NodeId},
    },
    utils::shared::Shared,
};

pub struct Graph {
    pub nodes: DashMap<NodeId, Shared<Node>>, // index by id
    pub edges: DashSet<Arc<Edge>>,

    // Label storage - These are only meant for getting nodes
    pub label_node_index: DashMap<String, DashSet<NodeId>>,
    pub label_edge_index: DashMap<String, DashSet<Arc<Edge>>>,

    // ID generator - uses Atomic64
    pub node_idgen: IdGenerator,

    pub store: Store,
}

impl Graph {
    pub fn new() -> Self {
        let nodes: DashMap<NodeId, Shared<Node>> = DashMap::new();
        let edges: DashSet<Arc<Edge>> = DashSet::new();
        let label_node_index: DashMap<String, DashSet<NodeId>> = DashMap::new();
        let label_edge_index: DashMap<String, DashSet<Arc<Edge>>> = DashMap::new();
        let store = Store::new(
                "/home/ahaandesai/programming/Projects/graphdb/src/persistence/store/nodes.dat".to_string(),
                "/home/ahaandesai/programming/Projects/graphdb/src/persistence/store/edges.dat".to_string()
            );

        let last_id = read_nodes_from_store(&nodes, &label_node_index, &store);
        read_edges_from_store(&edges, &label_edge_index, &store);


        Self {
            nodes,
            edges,
            label_node_index,
            label_edge_index,
            node_idgen: IdGenerator::new(last_id + 1),
            store
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

fn read_nodes_from_store(
    node_index: &DashMap<NodeId, Shared<Node>>,
    label_node_index: &DashMap<String, DashSet<NodeId>>,
    store: &Store,
) -> NodeId {
    let nodes = store.read_nodes().unwrap();
    let mut last_id = 0;

    for node in nodes {
        let id = node.id;
        last_id = id;
        let label = node.label.clone();
        let properties = node.property_map;
        add_node_internal(
            node_index,
            label_node_index,
            label.as_str(),
            &properties,
            id,
        );
    }

    last_id
}

fn add_node_internal(
    nodes: &DashMap<NodeId, Shared<Node>>,
    label_node_index: &DashMap<String, DashSet<NodeId>>,
    label: &str,
    properties: &PropertyMap,
    id: NodeId,
) -> NodeId {
    let node = shared(Node::new(id, label.to_string(), properties.clone()));

    nodes.insert(id, node);

    let entry = label_node_index
        .entry(label.to_string())
        .or_insert_with(DashSet::new);

    entry.insert(id);

    id
}

fn read_edges_from_store(
    edge_index: &DashSet<Arc<Edge>>,
    label_edge_index: &DashMap<String, DashSet<Arc<Edge>>>,
    store: &Store,
) {
    let edges = store.read_edges().unwrap();

    for edge in edges {
        let label = edge.label.clone();
        let arc_edge = Arc::new(edge);
        edge_index.insert(arc_edge.clone());

        let entry = label_edge_index
            .entry(label.to_string())
            .or_insert_with(DashSet::new);

        entry.insert(arc_edge);
    }
}