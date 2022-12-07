fn status_out(event: &str, checkpoint: usize, target: usize, chunks: usize) {
    if checkpoint % chunks == 0 || checkpoint == target {
        println!("Status ({}): {}/{}.", event, checkpoint, target);
    }
}

fn sieve_of_eratosthenes(max_candidate: usize) -> Vec<usize> {
    let mut potential_primes = vec![true; max_candidate + 1];
    potential_primes[0] = false;
    potential_primes[1] = false;

    let max_factor = ((max_candidate as f64).sqrt() + 1.0) as usize;
    for candidate in 2..=max_factor {
        status_out("enumeration", candidate, max_factor, 1000);
        if potential_primes[candidate] {
            let mut index_of_multiple = usize::pow(candidate, 2);
            while index_of_multiple <= max_candidate {
                potential_primes[index_of_multiple] = false;
                index_of_multiple += candidate;
            }
        }
    }

    let mut primes: Vec<usize> = Vec::new();
    let mut index = max_candidate;
    while index >= 2 {
        status_out("reduction", index, 2, 100_000);
        if potential_primes[index] {
            primes.push(index);
        }

        potential_primes.pop();
        index -= 1;
    }

    primes.reverse();
    primes
}

pub fn less_than(number: usize) -> Vec<usize> {
    match number {
        0 | 1 => Vec::new(),
        _ => sieve_of_eratosthenes(number),
    }
}
