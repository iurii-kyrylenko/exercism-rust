pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: PartialOrd,
    S: AsRef<[T]>,
{
    find_with_pos(array.as_ref(), key, 0)
}

fn find_with_pos<T>(slice: &[T], key: T, pos: usize) -> Option<usize>
where
    T: PartialOrd,
{
    let mid = slice.len() / 2;

    let check = slice.get(mid)?;

    if *check == key {
        return Some(pos + mid);
    }

    let (left, right) = slice.split_at(mid);

    if *check < key {
        find_with_pos(&right[1..], key, pos + mid + 1)
    } else {
        find_with_pos(left, key, pos)
    }
}
