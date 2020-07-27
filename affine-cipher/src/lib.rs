/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const N: i32 = 26; // a..z
const CODE_BASE: i32 = 97; // a
const SPACES: usize = 5;

type ReIndex = fn(i32, i32, i32) -> i32;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    mmi(a, N)?;

    Ok(with_spacing(cypher_iterator(plaintext, encode_index, a, b)).collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mmi = mmi(a, N)?;

    Ok(cypher_iterator(ciphertext, decode_index, mmi, b).collect())
}

// Returns an iterator on processed text
fn cypher_iterator(
    text: &str,
    reindex: ReIndex,
    a: i32,
    b: i32,
) -> impl Iterator<Item = char> + '_ {
    text.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(move |c| match c {
            _ if c.is_digit(10) => c,
            _ => {
                let idx = c.to_ascii_lowercase() as i32 - CODE_BASE;
                (reindex(idx, a, b) + CODE_BASE) as u8 as char
            }
        })
}

// Inserts spacing into iterator for processed text
// https://stackoverflow.com/questions/57029974/how-to-split-string-into-chunks-in-rust-to-insert-spaces
fn with_spacing(iterator: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
    iterator.enumerate().flat_map(|(i, c)| {
        if i != 0 && i % SPACES == 0 {
            Some(' ')
        } else {
            None
        }
        .into_iter()
        .chain(std::iter::once(c))
    })
}

// Calculates a modular mulplicative inverse
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn mmi(a: i32, n: i32) -> Result<i32, AffineCipherError> {
    let mut t = (0, 1);
    let mut r = (n, a);

    while r.1 != 0 {
        let quotient = r.0 / r.1;
        t = (t.1, t.0 - quotient * t.1);
        r = (r.1, r.0 - quotient * r.1);
    }

    if r.0 > 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(if t.0 < 0 { t.0 + n } else { t.0 })
    }
}

// Re-index when encoding
fn encode_index(idx: i32, a: i32, b: i32) -> i32 {
    (a * idx + b).rem_euclid(N)
}

// Re-index when decoding
fn decode_index(idx: i32, mmi: i32, b: i32) -> i32 {
    (mmi * (idx - b)).rem_euclid(N)
}
