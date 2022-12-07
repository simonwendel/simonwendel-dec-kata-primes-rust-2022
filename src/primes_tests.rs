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
