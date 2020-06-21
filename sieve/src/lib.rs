pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = upper_bound as usize + 1;

    let limit = (n as f64).sqrt() as usize + 1;

    let mut checks = vec![true; n];

    for i in 2..limit {
        step(&mut checks, i);
    }

    checks
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| match c {
            false => None,
            true => Some(i as u64),
        })
        .skip(2)
        .collect()
}

fn step(a: &mut [bool], i: usize) {
    let mut j = i * i;

    while j < a.len() {
        a[j] = false;
        j += i;
    }
}
