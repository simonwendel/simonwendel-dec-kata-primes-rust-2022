
pub mod primes;

#[cfg(test)]
mod tests {
    use super::primes;

    #[test]
    fn mod_primes_fn_get_given_zero_returns_no_primes() {
        let primes = primes::get(0);
        assert_eq!(0, primes.len());
    }
}