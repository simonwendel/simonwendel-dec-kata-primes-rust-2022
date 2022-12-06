use rusty_primes::primes;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.push(String::from(""));

    let mut max_candidate = 17179869183_usize; // this default uses up to ~21GB of RAM to run the sieve
    match args[1].parse::<usize>() {
        Ok(input_candidate) => {
            max_candidate = input_candidate;
            println!(
                "Using supplied value of {} as max_candidate.",
                max_candidate
            );
        }
        Err(_) => {
            println!("Using default value of {} as max_candidate.", max_candidate);
        }
    };

    let primes = primes::get_up_until(max_candidate);
    let next_to_last_prime = primes[primes.len() - 2];
    let last_prime = primes[primes.len() - 1];

    println!("Found {} primes.", primes.len());
    println!(
        "Last primes in the series:\n{} * {} = {}",
        next_to_last_prime,
        last_prime,
        next_to_last_prime * last_prime
    );
}
