use std::collections::BTreeMap;
use std::ops::Range;

fn main() {
    let map = get_palindromes_in_range(1000..10000);

    let len = map.len();
    println!("len = {}", len);

    for x in map
        .iter()
        .enumerate()
        .filter(|(i, _)| i == &0 || i == &(len - 1))
    {
        println!("{:?}", x);
    }

    // println!("{:?}", (map.iter().next(), map.iter().last()));
}

fn get_palindromes_in_range(r: Range<usize>) -> BTreeMap<usize, Vec<(usize, usize)>> {
    let mut map = BTreeMap::new();

    r.clone()
        .map(|i| r.clone().map(move |j| (i, j)).filter(|(i, j)| j >= i))
        .flatten()
        .map(|(i, j)| (i, j, i * j))
        .filter(|x| is_palindrome(x.2))
        .for_each(|x| map.entry(x.2).or_insert(vec![]).push((x.0, x.1)));

    map
}

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
