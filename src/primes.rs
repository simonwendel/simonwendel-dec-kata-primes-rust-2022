fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut all_integers = (0..=n).into_iter().map(|_| true).collect::<Vec<_>>();
    all_integers[0] = false;
    all_integers[1] = false;

    let max_factor = ((n as f64).sqrt() + 1.0) as usize;

    for number in 2..=max_factor {
        if all_integers[number] {
            let mut j = usize::pow(number, 2);
            while j <= n {
                all_integers[j] = false;
                j += number;
            }
        }
    }
    
    let primes = all_integers.into_iter().enumerate().filter_map(|(i, val)| if val { Some(i) } else { None} );
    primes.collect()
}

pub fn get_up_until(max_number: usize) -> Vec<usize> {
    match max_number {
        0 | 1 => Vec::new(),
        _ => sieve_of_eratosthenes(max_number)
    }
}
