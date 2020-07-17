type ScrabbleMap = std::collections::HashMap<char, u64>;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    with_map(get_map, word)
}

fn get_map() -> ScrabbleMap {
    vec![
        ('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1), ('l', 1), ('n', 1),
        ('r', 1), ('s', 1), ('t', 1), ('d', 2), ('g', 2), ('b', 3), ('c', 3),
        ('m', 3), ('p', 3), ('f', 4), ('h', 4), ('v', 4), ('w', 4), ('y', 4),
        ('k', 5), ('j', 8), ('x', 8), ('q', 10), ('z', 10),
    ]
    .into_iter()
    .collect()
}
    
fn with_map(get_map: fn() -> ScrabbleMap, word: &str) -> u64 {
    let hm = get_map();

    word.chars()
        .map(|c| match hm.get(&c.to_ascii_lowercase()) {
            Some(score) => score,
            _ => &0,
        })
        .sum()
}
