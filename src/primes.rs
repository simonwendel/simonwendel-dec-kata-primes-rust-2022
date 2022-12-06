fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut all_integers = (0..=n).into_iter().map(Some).collect::<Vec<_>>();
    all_integers[0] = None;
    all_integers[1] = None;

    let max_factor = ((n as f32).sqrt() + 1.0) as usize;

    for number in 2..=max_factor {
        if all_integers[number].is_some() {
            let mut j = usize::pow(number, 2);
            while j <= n {
                all_integers[j] = None;
                j += number;
            }
        }
    }

    all_integers.into_iter().flatten().collect()
}

pub fn get_up_until(max_number: usize) -> Vec<usize> {
    match max_number {
        0 | 1 => Vec::new(),
        _ => sieve_of_eratosthenes(max_number)
    }
}
