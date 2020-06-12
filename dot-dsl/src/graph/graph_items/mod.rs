pub mod edge;
pub mod node;

use std::collections::HashMap;

pub fn to_hash_map(attrs: &[(&str, &str)]) -> HashMap<String, String> {
    attrs
        .iter()
        .map(|(fst, snd)| (String::from(*fst), String::from(*snd)))
        .collect()
}
