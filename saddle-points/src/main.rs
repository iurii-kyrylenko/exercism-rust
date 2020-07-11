use std::collections::{BTreeMap, BTreeSet};

fn get_max(input: &[Vec<u64>]) -> BTreeSet<(usize, usize)> {
    let rows = input.len();
    let cols = input[0].len();
    let mut result = BTreeSet::new();

    for i in 0..rows {
        let mut map: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::new();

        for j in 0..cols {
            let value = input[i][j];
            map.entry(value).or_insert(vec![]).push((i, j));
        }

        for cell in map.values().last().unwrap() {
            result.insert(*cell);
        }
    }

    result
}

fn get_min(input: &[Vec<u64>]) -> BTreeSet<(usize, usize)> {
    let rows = input.len();
    let cols = input[0].len();
    let mut result = BTreeSet::new();

    for j in 0..cols {
        let mut map: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::new();

        for i in 0..rows {
            let value = input[i][j];
            map.entry(value).or_insert(vec![]).push((i, j));
        }

        for cell in map.values().next().unwrap() {
            result.insert(*cell);
        }
    }

    result
}

fn main() {
    let input = vec![vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
    let maxs = get_max(&input);
    let mins = get_min(&input);
    let res: Vec<(usize, usize)> = maxs.intersection(&mins).cloned().collect();
    println!("{:?}", res);
}
