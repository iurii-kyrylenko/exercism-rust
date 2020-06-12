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
