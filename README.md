# RustyPrimes - A Simple Sieve in Rust

Finding prime numbers is always fun. This is the sieve of Eratosthenes in Rust. Being a sieve it is of course hugely
memory intensive. Running time was never a priority since the amount of RAM would be the limiting factor. Finding all
primes less than 125.000.000.000 yielded the following results on a VM with 20 EPYC cores and 160GB RAM:

```
Found 5100605440 primes.
Last primes in the series:
124999999981 * 124999999997 = 15624999997250000000057
```

It completed in a little over 20 minutes. The 20 cores were of course a bit of an overkill, since the program is strictly
single-threaded, but it was the best VM size in Azure I could get with respect to RAM size without raising a quota request,
and who has time for that?

## Run environment

* Rust development machine, preferrably 64-bit
    * There's a VS Code dev container config in the repo
* A host to run the sieve on, a big one

## How to use

1. Do a quick `cargo build --release` on the dev machine
2. Run the `rusty_primes` executable (or `rusty_primes.exe` if you're into that)
3. Wait for the results
    * This will take a lot of time if you're aiming high
5. Success (or maybe a panic)!

## Note on picking a target

Make sure you aim for a target a bit smaller than your available system memory to allow for filtering. You can overshoot
your actual physical memory by a bit, but that would hit the swap and you will have to wait a whole lot longer for 
your results (unless of course you're 1337 and always turn off swap in which case you will get a segfault on GNU/Linux
and a BSOD on Windows).

## License

All code licensed under GPL-3.0-only, license text in COPYING file in the repo root.

## Have fun

Please do.
