mod mod_arith;
mod sieve;

fn main() {
    let n = 1_000_000_000;
    let start = std::time::Instant::now();
    //let primes = sieve::sieve_of_eratosthenes_bitset(n);
    //let primes = sieve::segment_sieve_wheel(n);
    //let primes = sieve::segmented_sieve_with_wheel(n);
    let primes = sieve::segmented_sieve_with_bitset(n);
    //let primes = sieve::sieve_of_eratosthenes_bitset(n);
    let duration = start.elapsed();
    println!(
        "Running for n: {}\nTotal primes: {:?} numbers, Time: {:?}",
        n,
        primes.len(),
        duration
    );
}
