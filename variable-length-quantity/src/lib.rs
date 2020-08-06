use std::iter;

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

    if let Some([]) = groups.clone().last() {
        let ends = bytes.iter().filter(|&byte| byte & 0x80 == 0);

        let result = groups
            .zip(ends)
            .map(bytes_to_number)
            .collect::<Vec<_>>();

        Ok(result)
    } else {
        Err(Error::IncompleteNumber)
    }
}

pub fn number_to_bytes(value: &u32) -> Vec<u8> {
    if value == &0 {
        return vec![0];
    }

    let mut res = iter::successors(parts(*value, 0x00), |(fst, _)| parts(*fst, 0x80))
        .map(|(_, snd)| snd)
        .collect::<Vec<_>>();

    res.reverse();
    res
}

fn bytes_to_number(tuple: (&[u8], &u8)) -> u32 {
    println!("{:?}", tuple); //===========
    let result = tuple.0
        .iter()
        .fold(0_u32, |acc, byte| {
            print!("acc = {:x}, byte = {:x},", acc, byte & 0x7f);
            let res = (acc << 7) + (*byte & 0x7f) as u32;
            println!(" new_acc = {:x}", res);
            res
        });

    println!("acc = {:x}, byte = {:x},", result, *tuple.1);
    (result << 7) + *tuple.1 as u32
}

// fn bytes_to_number(tuple: (&[u8], &u8)) -> Result<u32, Error> {
//     let result = tuple.0
//         .iter()
//         .fold(0_u32, |acc, byte| (acc << 7) + (*byte & 0x7f) as u32);

//     Ok((result << 7) + *tuple.1 as u32)
// }

fn parts(value: u32, mask: u8) -> Option<(u32, u8)> {
    if value == 0 {
        None
    } else {
        let fst = value >> 7;
        let snd = value as u8 & 0x7f | mask;
        Some((fst, snd))
    }
}
