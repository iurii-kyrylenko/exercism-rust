use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

struct Counts {
    map: HashMap<char, usize>,
}

impl Counts {
    fn new() -> Self {
        Counts {
            // Option: use macro maplit::hashmap!
            map: vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)]
                .into_iter()
                .collect(),
        }
    }

    fn handle_char(&mut self, c: char) -> Result<(), char> {
        match self.map.entry(c).and_modify(|v| *v += 1) {
            Occupied(_) => Ok(()),
            Vacant(_) => Err(c),
        }
    }

    fn handle_str(&mut self, s: &str) -> Result<(), char> {
        for c in s.chars() {
            self.handle_char(c)?;
        }

        Ok(())
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if "ACGT".chars().any(|c| c == nucleotide) {
        let hash_map = nucleotide_counts(dna)?;
        Ok(hash_map[&nucleotide])
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = Counts::new();
    let _ = counts.handle_str(dna)?;

    Ok(counts.map)
}
