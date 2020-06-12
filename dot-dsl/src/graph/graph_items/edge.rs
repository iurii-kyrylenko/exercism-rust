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
