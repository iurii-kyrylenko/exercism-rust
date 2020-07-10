use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Self {
        Self {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        let (a, b) = self.factors[0];
        a * b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }

    fn build(factors: &[(u64, u64)]) -> Self {
        Self {
            factors: factors.to_vec(),
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let map = get_palindromes_in_range(min..max + 1);

    if map.len() == 0 {
        return None;
    }

    Some((
        Palindrome::build(map.values().next().unwrap()),
        Palindrome::build(map.values().last().unwrap()),
    ))
}

// This is the main implementation.
// The code above serves basically for testing purposes.
// 
fn get_palindromes_in_range(r: Range<u64>) -> BTreeMap<u64, Vec<(u64, u64)>> {
    let mut map = BTreeMap::new();

    r.clone()
        .map(|i| r.clone().map(move |j| (i, j)).filter(|(i, j)| j >= i))
        .flatten()
        .map(|(i, j)| (i, j, i * j))
        .filter(|x| is_palindrome(x.2))
        .for_each(|x| map.entry(x.2).or_insert(vec![]).push((x.0, x.1)));

    map
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
