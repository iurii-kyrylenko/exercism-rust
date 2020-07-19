use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter_map(|c| match c.to_ascii_uppercase() {
            cap @ 'A'..='Z' => Some(cap),
            _ => None,
        })
        .collect::<HashSet<char>>()
        .len()
        == 26
}
