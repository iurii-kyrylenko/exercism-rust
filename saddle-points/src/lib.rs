// use std::collections::{BTreeMap, BTreeSet};

// pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
//     let rows = input.len();
//     if rows == 0 {
//         return vec![];
//     }

//     let cols = input[0].len();
//     if cols == 0 {
//         return vec![];
//     }

//     let maxs = get_maxs(rows, cols, &input);
//     let mins = get_mins(rows, cols, &input);

//     maxs.intersection(&mins).cloned().collect()
// }

// fn get_maxs(rows: usize, cols: usize, input: &[Vec<u64>]) -> BTreeSet<(usize, usize)> {
//     let mut result = BTreeSet::new();

//     for i in 0..rows {
//         let mut map: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::new();

//         for j in 0..cols {
//             map.entry(input[i][j]).or_insert(vec![]).push((i, j));
//         }

//         for cell in map.values().last().unwrap() {
//             result.insert(*cell);
//         }
//     }

//     result
// }

// fn get_mins(rows: usize, cols: usize, input: &[Vec<u64>]) -> BTreeSet<(usize, usize)> {
//     let mut result = BTreeSet::new();

//     for j in 0..cols {
//         let mut map: BTreeMap<u64, Vec<(usize, usize)>> = BTreeMap::new();

//         for i in 0..rows {
//             map.entry(input[i][j]).or_insert(vec![]).push((i, j));
//         }

//         for cell in map.values().next().unwrap() {
//             result.insert(*cell);
//         }
//     }

//     result
// }

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for (x, row) in input.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if value >= row.iter().max().unwrap()
                && *value <= input.iter().map(|row| row[y]).min().unwrap()
            {
                saddle_points.push((x, y));
            }
        }
    }

    saddle_points
}