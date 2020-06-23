use std::collections::{BTreeMap, BTreeSet};

pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> Self {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        Some(self.0.get(&grade)?.iter().map(|x| x.clone()).collect())
    }
}
