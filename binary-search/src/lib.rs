use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: Ord,
    S: AsRef<[T]>,
{
    find_with_pos(array.as_ref(), key, 0)
}

fn find_with_pos<T>(slice: &[T], key: T, pos: usize) -> Option<usize>
where
    T: Ord,
{
    let mid = slice.len() / 2;
    let check = slice.get(mid)?;
    let (left, right) = slice.split_at(mid);

    match check.cmp(&key) {
        Equal => Some(pos + mid),
        Less => find_with_pos(&right[1..], key, pos + mid + 1),
        Greater => find_with_pos(left, key, pos),
    }
}
