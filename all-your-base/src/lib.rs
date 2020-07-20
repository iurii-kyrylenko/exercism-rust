use std::iter;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match (from_base, to_base) {
        (0..=1, _) => Err(Error::InvalidInputBase),
        (_, 0..=1) => Err(Error::InvalidOutputBase),
        _ => {
            let parsed = parse(number, from_base)?;
            Ok(to_digits(parsed, to_base))
        }
    }
}

fn parse(number: &[u32], from_base: u32) -> Result<u32, Error> {
    number.iter().try_fold(0, |acc, &d| {
        if d >= from_base {
            Err(Error::InvalidDigit(d))
        } else {
            Ok(d + from_base * acc)
        }
    })
}

fn to_digits(n: u32, base: u32) -> Vec<u32> {
    if n == 0 {
        return vec![0];
    }

    let mut digits: Vec<u32> = iter::successors(div_mod(n, base), |(d, _)| div_mod(*d, base))
        .map(|(_, m)| m)
        .collect();

    digits.reverse();

    digits
}

fn div_mod(n: u32, base: u32) -> Option<(u32, u32)> {
    if n > 0 {
        Some((n / base, n % base))
    } else {
        None
    }
}
