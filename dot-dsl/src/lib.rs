pub mod graph {
    use std::collections::HashMap;

    pub struct Graph {
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,

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
                attrs: attrs
                    .iter()
                    .map(|(fst, snd)| (String::from(*fst), String::from(*snd)))
                    .collect(),
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            }
        }

        pub fn with_nodes(&self, nodes: &[graph_items::node::Node]) -> Self {
            Graph {
                attrs: self.attrs.clone(),
                nodes: nodes.to_vec(),
                edges: self.edges.clone(),
            }
        }

        pub fn with_edges(&self, edges: &[graph_items::edge::Edge]) -> Self {
            Graph {
                attrs: self.attrs.clone(),
                nodes: self.nodes.clone(),
                edges: edges.to_vec(),
            }
        }

        pub fn get_node(&self, label: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|node| node.label == label)
        }
    }

    pub mod graph_items {
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
                        attrs: attrs
                            .iter()
                            .map(|(fst, snd)| (String::from(*fst), String::from(*snd)))
                            .collect(),
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
                        attrs: attrs
                            .iter()
                            .map(|(fst, snd)| (String::from(*fst), String::from(*snd)))
                            .collect(),
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
