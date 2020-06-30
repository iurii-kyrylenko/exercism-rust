use permutator::KPermutationIterator;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Alphametic<'a> {
    terms: Vec<&'a str>,
    map: HashMap<char, u8>,
}

impl<'a> Alphametic<'a> {
    fn new(input: &'a str) -> Self {
        let re = Regex::new(r"\s*(\+|==)\s*").unwrap();
        let terms: Vec<&'a str> = re.split(input.trim()).collect();

        let map = terms
            .iter()
            .map(|t| t.chars())
            .flatten()
            .map(|c| (c, 0))
            .collect();

        Self { terms, map }
    }

    fn update_map(&mut self, numbers: Vec<&u8>) {
        for (key, number) in self.map.clone().keys().zip(numbers) {
            self.map.insert(*key, *number);
        }
    }

    fn to_number(&self, i: usize, term: &str) -> i64 {
        let number = term
            .chars()
            .fold(0, |acc, c| 10 * acc + self.map[&c] as i64);
        if i == self.terms.len() - 1 {
            -number
        } else {
            number
        }
    }

    fn check(&self) -> bool {
        let sum = self
            .terms
            .iter()
            .enumerate()
            .map(|(i, term)| self.to_number(i, term))
            .sum::<i64>();
        sum == 0
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut solver = Alphametic::new(input);
    let len = solver.map.len();
    let data: Vec<u8> = (0..10 as u8).collect();
    let permutator = KPermutationIterator::new(&data, len);

    for numbers in permutator {
        solver.update_map(numbers);

        match solver.check() {
            true => return Some(solver.map),
            _ => continue,
        }
    }

    None
}
