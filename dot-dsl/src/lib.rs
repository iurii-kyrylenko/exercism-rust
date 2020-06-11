pub mod graph {
    pub struct Graph {
        pub attrs: Vec<(String, String)>,
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,

    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                attrs: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
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
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug)]
            pub struct Node {
                label: String,
                attrs: Vec<(String, String)>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Node {
                        label: String::from(label),
                        attrs: Vec::new(),
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
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.label == other.label
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Debug)]
            pub struct Edge {
                label: (String, String),
            }

            impl Edge {
                pub fn new(fst: &str, snd: &str) -> Self {
                    Edge {
                        label: (String::from(fst), String::from(snd)),
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
