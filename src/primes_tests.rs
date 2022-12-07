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

use super::primes;

#[test]
fn mod_primes_fn_less_than_given_0_returns_no_primes() {
    let primes = primes::less_than(0);
    assert_eq!(0, primes.len());
}

#[test]
fn mod_primes_fn_less_than_given_1_returns_no_primes() {
    let primes = primes::less_than(1);
    assert_eq!(0, primes.len());
}

#[test]
fn mod_primes_fn_less_than_given_2_returns_1_prime() {
    let primes = primes::less_than(2);
    assert_eq!(vec![2], primes);
}

#[test]
fn mod_primes_fn_less_than_given_30_returns_10_first_primes() {
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let actual = primes::less_than(30);
    assert_eq!(expected, actual);
}

#[test]
fn mod_primes_fn_firstn_given_10_returns_10_first_primes() {
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let actual = primes::firstn(10);
    assert_eq!(expected, actual);
}

#[test]
fn mod_primes_fn_firstn_given_25_returns_25_first_primes() {
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let actual = primes::firstn(25);
    assert_eq!(expected, actual);
}
