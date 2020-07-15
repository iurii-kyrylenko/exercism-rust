use std::iter;

pub fn encode(n: u64) -> String {
    if n == 0 {
        return say_zero().to_string();
    }

    let ts: Vec<(usize, u64)> = iter::successors(div_mod(n), |&(d, _)| div_mod(d))
        .enumerate()
        .filter(|&(_, (_, m))| m > 0)
        .map(|(i, (_, m))| (i, m))
        .collect();

    ts.iter()
        .rev()
        .map(|&(exp, triple)| join_str(&say_triple(triple), say_exp(exp), " "))
        .collect::<Vec<_>>()
        .join(" ")
}

fn div_mod(n: u64) -> Option<(u64, u64)> {
    if n > 0 {
        Some((n / 1000, n % 1000))
    } else {
        None
    }
}

fn join_str(s1: &str, s2: &str, delimiter: &str) -> String {
    let sep = if s1 == "" || s2 == "" { "" } else { delimiter };
    format!("{}{}{}", s1, sep, s2)
}

fn say_triple(triple: u64) -> String {
    let hs = triple / 100;
    let tuple = triple % 100;

    join_str(&say_hundreds(hs), &say_tuple(tuple), " ")
}

fn say_hundreds(hs: u64) -> String {
    if hs == 0 {
        "".to_string()
    } else {
        join_str(say_single(hs), say_hundred(), " ")
    }
}

fn say_tuple(tuple: u64) -> String {
    let ds = tuple / 10;
    let sn = tuple % 10;

    match (ds, sn) {
        (0, _) => say_single(sn).to_string(),
        (1, _) => say_teen(sn).to_string(),
        (_, 0) => say_ty(ds).to_string(),
        _ => join_str(say_ty(ds), say_single(sn), "-"),
    }
}

fn say_zero() -> &'static str {
    "zero"
}

fn say_hundred() -> &'static str {
    "hundred"
}

fn say_exp(exp: usize) -> &'static str {
    match exp {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => "",
    }
}

fn say_single(single: u64) -> &'static str {
    match single {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }
}

fn say_teen(single: u64) -> &'static str {
    match single {
        0 => "ten",
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        9 => "nineteen",
        _ => "",
    }
}

fn say_ty(ds: u64) -> &'static str {
    match ds {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
}
