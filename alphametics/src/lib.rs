use permutator::KPermutationIterator;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

struct Alphametic {
    keys: String,
    term_map: HashMap<String, isize>,
}

impl Alphametic {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"\s*(\+|==)\s*").unwrap();
        let terms: Vec<&str> = re.split(input.trim()).collect();
        let last_term_id = terms.len() - 1;

        let set: HashSet<char> = terms.iter().map(|t| t.chars()).flatten().collect();
        let keys: String = set.iter().collect();

        let mut term_map: HashMap<String, isize> = HashMap::new();

        for (i, term) in terms.iter().enumerate() {
            let delta = if i == last_term_id { -1 } else { 1 };
            let count = term_map.entry(term.to_string()).or_insert(0);
            *count += delta;
        }

        Self { term_map, keys }
    }

    fn char_map(&self, numbers: Vec<&u8>) -> HashMap<char, u8> {
        self.keys.chars().zip(numbers.iter().map(|n| **n)).collect()
    }

    fn to_number(term: &str, map: &HashMap<char, u8>) -> isize {
        term.chars().fold(0, |acc, c| 10 * acc + map[&c] as isize)
    }

    fn is_lead_zero(&self, map: &HashMap<char, u8>) -> bool {
        self.term_map
            .keys()
            .map(|key| key.chars().nth(0))
            .flatten()
            .any(|c| map[&c] == 0)
    }

    fn check(&self, map: &HashMap<char, u8>) -> bool {
        let sum = self
            .term_map
            .iter()
            .map(|(term, i)| Alphametic::to_number(term, map) * i)
            .sum::<isize>();

        sum == 0 && !self.is_lead_zero(map)
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let timer = Instant::now();
    let solver = Alphametic::new(input);
    let data: Vec<u8> = (0..10 as u8).collect();
    let permutator = KPermutationIterator::new(&data, solver.keys.len());

    for numbers in permutator {
        let map = solver.char_map(numbers);

        match solver.check(&map) {
            true => {
                println!("========= done in {:?} ============", timer.elapsed());
                return Some(map);
            }
            _ => continue,
        }
    }

    println!("========= done in {:?} ============", timer.elapsed());
    None
}
