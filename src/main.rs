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

use rusty_primes::primes;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.push(String::from(""));

    if let Ok(max_candidate) = args[1].parse::<usize>() {
        println!(
            "Will attempt to use {} as initial max_candidate.",
            max_candidate
        );

        let primes = primes::less_than(max_candidate);
        let next_to_last_prime = primes[primes.len() - 2] as u128;
        let last_prime = primes[primes.len() - 1] as u128;

        println!("Found {} primes.", primes.len());
        println!(
            "Last primes in the series:\n{} * {} = {}",
            next_to_last_prime,
            last_prime,
            next_to_last_prime * last_prime
        );
    } else {
        println!("Usage: rusty_primes <number>");
    }
}
