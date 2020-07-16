use regex::Regex;
use std::iter;

pub fn encode(source: &str) -> String {
    let chars_1 = iter::once('_').chain(source.chars());
    let chars_2 = source.chars().chain(iter::once('_'));

    let itr = chars_1.zip(chars_2).enumerate().filter(|(_, p)| p.0 != p.1);

    itr.clone()
        .zip(itr.clone().skip(1))
        .map(|(f, s)| {
            let n = s.0 - f.0;
            let c = (s.1).0;
            if n > 1 {
                format!("{}{}", n, c)
            } else {
                format!("{}", c)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn decode(source: &str) -> String {
    let re = Regex::new(r"(?P<count>\d*)(?P<char>[A-Za-z ])").unwrap();

    re.captures_iter(source)
        .map(|caps| {
            let count = match caps["count"].parse() {
                Ok(res) => res,
                _ => 1,
            };

            caps["char"].repeat(count)
        })
        .collect::<Vec<_>>()
        .join("")
}
