use rusty_primes::primes;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.push(String::from(""));

    let max_candidate = args[1].parse::<usize>().unwrap_or(17179869183_usize);
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
}
