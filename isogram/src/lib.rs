use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let string: String = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    string.len() == string.chars().collect::<HashSet<_>>().len()
}
