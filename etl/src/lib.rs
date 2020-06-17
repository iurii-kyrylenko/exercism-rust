use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (score, chars) in h.into_iter() {
        for y in chars {
            // result.insert(y.to_ascii_lowercase(), *score);
            result.insert(y.to_lowercase().nth(0).unwrap(), *score);
        }
    }

    result
}
