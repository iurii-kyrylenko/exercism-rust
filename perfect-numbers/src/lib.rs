use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        Some(
            match (1..=num / 2)
                .filter(|f| num % f == 0)
                .sum::<u64>()
                .cmp(&num)
            {
                Ordering::Greater => Classification::Abundant,
                Ordering::Equal => Classification::Perfect,
                Ordering::Less => Classification::Deficient,
            },
        )
    }
}
