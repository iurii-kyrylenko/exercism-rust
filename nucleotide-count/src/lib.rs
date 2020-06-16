use std::collections::HashMap;

struct Counts {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl Counts {
    fn new() -> Self {
        Counts {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
        }
    }

    fn handle_char(&mut self, c: char) -> Result<(), char> {
        match c {
            'A' => Ok(self.a += 1),
            'C' => Ok(self.c += 1),
            'G' => Ok(self.g += 1),
            'T' => Ok(self.t += 1),
            _ => Err(c),
        }
    }

    fn handle_str(&mut self, s: &str) -> Result<(), char> {
        for c in s.chars() {
            self.handle_char(c)?;
        }

        Ok(())
    }

    fn to_hash_map(&self) -> HashMap<char, usize> {
        vec![('A', self.a), ('C', self.c), ('G', self.g), ('T', self.t)]
            .into_iter()
            .collect()
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

    Ok(counts.to_hash_map())
}
