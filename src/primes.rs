// RustyPrimes - A Simple Sieve in Rust
// Copyright (C) 2022  Simon Wendel
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub fn less_than(n: usize) -> Vec<usize> {
    match n {
        0 | 1 => Vec::new(),
        _ => sieve_of_eratosthenes(n, None),
    }
}

pub fn firstn(n: usize) -> Vec<usize> {
    let upper_candidate = match n {
        0..=5 => 11,
        _ => dusart_upper_bound(n),
    };

    sieve_of_eratosthenes(upper_candidate, Some(n))
}

fn dusart_upper_bound(n: usize) -> usize {
    let f = n as f64;
    (f * (f.ln() + f.ln().ln())) as usize
}

fn sieve_of_eratosthenes(upper_candidate: usize, pick_n: Option<usize>) -> Vec<usize> {
    let sieve = construct_sieve(upper_candidate);
    let primes = reduce_sieve(sieve);
    select_primes(primes, pick_n)
}

fn construct_sieve(upper_candidate: usize) -> Vec<bool> {
    let mut sieve = vec![true; upper_candidate + 1];
    sieve[0] = false;
    sieve[1] = false;

    let upper_factor = ((upper_candidate as f64).sqrt() + 1.0) as usize;
    for candidate in 2..=upper_factor {
        status_out("enumeration", candidate, upper_factor, 1000);
        if sieve[candidate] {
            let mut integer_to_cross_out = usize::pow(candidate, 2);
            while integer_to_cross_out <= upper_candidate {
                sieve[integer_to_cross_out] = false;
                integer_to_cross_out += candidate;
            }
        }
    }

    sieve
}

fn reduce_sieve(mut sieve: Vec<bool>) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();
    let mut index = sieve.len() - 1;
    while index >= 2 {
        status_out("reduction", index, 2, 100_000);
        if sieve[index] {
            primes.push(index);
        }

        sieve.pop();
        index -= 1;
    }

    primes
}

fn select_primes(mut primes: Vec<usize>, pick_n: Option<usize>) -> Vec<usize> {
    primes.reverse();
    match pick_n {
        Some(n) => primes.into_iter().take(n).collect(),
        _ => primes,
    }
}

fn status_out(event: &str, checkpoint: usize, target: usize, chunks: usize) {
    if checkpoint % chunks == 0 || checkpoint == target {
        println!("Status ({}): {}/{}.", event, checkpoint, target);
    }
}
