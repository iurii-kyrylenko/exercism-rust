use std::iter::successors;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(number_to_bytes).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let groups = bytes.split(|byte| byte & 0x80 == 0);

    if groups
        .clone()
        .any(|group| group.len() > 3 && group[0] > 0x8f)
    {
        Err(Error::Overflow)
    } else if let Some([]) = groups.clone().last() {
        let ends = bytes.iter().filter(|&byte| byte & 0x80 == 0);
        Ok(groups.zip(ends).map(bytes_to_number).collect())
    } else {
        Err(Error::IncompleteNumber)
    }
}

pub fn number_to_bytes(value: &u32) -> Vec<u8> {
    if value == &0 {
        vec![0]
    } else {
        let mut res = successors(parts(*value, 0x00), |(fst, _)| parts(*fst, 0x80))
            .map(|(_, snd)| snd)
            .collect::<Vec<_>>();

        res.reverse();
        res
    }
}

fn parts(value: u32, mask: u8) -> Option<(u32, u8)> {
    if value == 0 {
        None
    } else {
        let fst = value >> 7;
        let snd = value as u8 & 0x7f | mask;
        Some((fst, snd))
    }
}

fn bytes_to_number((group, end): (&[u8], &u8)) -> u32 {
    let res = group
        .iter()
        .fold(0_u32, |acc, byte| (acc << 7) + (*byte & 0x7f) as u32);

    (res << 7) + *end as u32
}
