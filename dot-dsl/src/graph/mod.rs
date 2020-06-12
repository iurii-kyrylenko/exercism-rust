pub mod graph_items;

use graph_items::edge::Edge;
use graph_items::node::Node;
use std::collections::HashMap;

pub struct Graph {
    pub attrs: HashMap<String, String>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            attrs: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
        Graph {
            attrs: graph_items::to_hash_map(attrs),
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }

    pub fn with_nodes(&self, nodes: &[Node]) -> Self {
        Graph {
            attrs: self.attrs.clone(),
            nodes: nodes.to_vec(),
            edges: self.edges.clone(),
        }
    }

    pub fn with_edges(&self, edges: &[Edge]) -> Self {
        Graph {
            attrs: self.attrs.clone(),
            nodes: self.nodes.clone(),
            edges: edges.to_vec(),
        }
    }

    pub fn get_node(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label == label)
    }
}
