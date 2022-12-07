fn status_out(checkpoint: usize, target: usize) {
    println!("Status: {}/{}.", checkpoint, target);
}

fn sieve_of_eratosthenes(max_candidate: usize) -> Vec<usize> {
    let mut potential_primes = vec![true; max_candidate + 1];
    potential_primes[0] = false;
    potential_primes[1] = false;

    let max_factor = ((max_candidate as f64).sqrt() + 1.0) as usize;
    for candidate in 2..=max_factor {
        if potential_primes[candidate] {
            let mut index_of_multiple = usize::pow(candidate, 2);
            while index_of_multiple <= max_candidate {
                potential_primes[index_of_multiple] = false;
                index_of_multiple += candidate;
            }
        }

        if candidate % 1000 == 0 || candidate == max_factor {
            status_out(candidate, max_factor);
        }
    }

    let primes = potential_primes
        .into_iter()
        .enumerate()
        .filter_map(|element| match element {
            (i, true) => Some(i),
            _ => None,
        });

    primes.collect()
}

pub fn less_than(max_number: usize) -> Vec<usize> {
    match max_number {
        0 | 1 => Vec::new(),
        _ => sieve_of_eratosthenes(max_number),
    }
}
