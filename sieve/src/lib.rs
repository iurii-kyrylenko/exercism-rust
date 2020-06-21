pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let n = upper_bound as usize + 1;

    let limit = (n as f64).sqrt() as usize + 1;

    let mut checks = vec![true; n];

    let step = |i| {
        (i * i..checks.len())
            .step_by(i)
            .for_each(|j| checks[j] = false)
    };

    (2..limit).for_each(step);

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
