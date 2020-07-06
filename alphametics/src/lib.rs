use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

struct Alphametic {
    keys: String,
    term_map: HashMap<Vec<usize>, isize>,
}

impl Alphametic {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"\s*(\+|==)\s*").unwrap();
        let terms: Vec<&str> = re.split(input.trim()).collect();

        let keys: String = terms.iter().map(|t| t.chars()).flatten().unique().collect();

        let keys_idx_map: HashMap<char, usize> = keys.chars().zip(0..10).collect();

        let mut term_map: HashMap<Vec<usize>, isize> = HashMap::new();

        for (i, term) in terms.iter().enumerate() {
            let term_as_idx = term.chars().map(|c| keys_idx_map[&c]).collect();
            let delta = if i == terms.len() - 1 { -1 } else { 1 };
            let count = term_map.entry(term_as_idx).or_insert(0);
            *count += delta;
        }

        Self { keys, term_map }
    }

    fn to_number(term: &Vec<usize>, perm: &Vec<usize>) -> isize {
        term.iter().fold(0, |acc, i| 10 * acc + perm[*i] as isize)
    }

    fn check(&self, perm: &Vec<usize>) -> bool {
        let sum = self
            .term_map
            .iter()
            .map(|(term, i)| Alphametic::to_number(term, perm) * i)
            .sum::<isize>();

        sum == 0 && !self.is_lead_zero(perm)
    }

    fn is_lead_zero(&self, perm: &Vec<usize>) -> bool {
        self.term_map.keys().map(|key| key[0]).any(|i| perm[i] == 0)
    }

    fn key_map(&self, perm: &Vec<usize>) -> HashMap<char, u8> {
        self.keys
            .chars()
            .zip(perm.iter().map(|&i| i as u8))
            .collect()
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let timer = Instant::now();
    let solver = Alphametic::new(input);
    let permutator = (0..10).permutations(solver.keys.len());

    for numbers in permutator {
        match solver.check(&numbers) {
            true => {
                println!("========= done in {:?} ============", timer.elapsed());
                return Some(solver.key_map(&numbers));
            }
            _ => continue,
        }
    }

    println!("========= done in {:?} ============", timer.elapsed());
    None
}
