#[macro_use]
extern crate lazy_static;
use regex::Regex;

// For the sake of simplicity and according to the tests we suppose
// that the input string has a single space as a word separator
// and all characters are alphabetic lowercase.
pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

// The 'regex' cargo doesn't support 'negative lookahead'.
// Otherwise we could use a single reg expression to get the starting
// consonant cluster like "^(?!xr|yt)y?((qu)?[^aeiouy]?(qu)?)*"
fn translate_word(word: &str) -> String {
    lazy_static! {
        static ref RE_1: Regex = Regex::new("^(xr|yt)").unwrap();
        static ref RE_2: Regex =
            Regex::new("^(?P<fst>y?((qu)?[^aeiouy]?(qu)?)*)?(?P<snd>.*)?").unwrap();
    }

    let word = if RE_1.is_match(word) {
        word.to_string()
    } else {
        RE_2.replace(word, "$snd$fst").to_string()
    };

    return format!("{}ay", word);
}
