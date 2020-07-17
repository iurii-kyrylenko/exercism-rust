/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let iter = isbn.chars().filter(|&c| c != '-');

    if iter.clone().count() != 10 {
        return false;
    }

    match iter.enumerate().fold(Some(0), |acc, (i, c)| {
        let d = get_digit(i, c)?;
        acc.map(|s| s + (10 - i as u32) * d)
    }) {
        None => false,
        Some(s) => s % 11 == 0,
    }
}

fn get_digit(pos: usize, c: char) -> Option<u32> {
    match (pos, c) {
        (9, 'X') => Some(10),
        _ => c.to_digit(10),
    }
}
