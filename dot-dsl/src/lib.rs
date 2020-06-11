pub mod graph {
    use std::collections::HashMap;
    use graph_items::node::Node;
    use graph_items::edge::Edge;

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

    pub mod graph_items {
        use std::collections::HashMap;

        pub fn to_hash_map(attrs: &[(&str, &str)]) -> HashMap<String, String> {
            attrs
                .iter()
                .map(|(fst, snd)| (String::from(*fst), String::from(*snd)))
                .collect()
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug)]
            pub struct Node {
                pub label: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        label: self.label.clone(),
                        attrs: super::to_hash_map(attrs),
                    }
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.label == other.label
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug)]
            pub struct Edge {
                label: (String, String),
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(fst: &str, snd: &str) -> Self {
                    Edge {
                        label: (String::from(fst), String::from(snd)),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        label: self.label.clone(),
                        attrs: super::to_hash_map(attrs),
                    }
                }
            }

            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    self.label == other.label
                }
            }
        }
    }
}
