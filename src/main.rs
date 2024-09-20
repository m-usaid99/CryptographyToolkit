mod mod_arith;
mod sieve;

fn main() {
    let n = 1_000_000_000;
    let start = std::time::Instant::now();
    let primes = sieve::parallel_segmented_sieve(n);
    let duration = start.elapsed();
    println!(
        "Running for n: {}\nTotal primes: {:?} numbers, Time: {:?}",
        n,
        primes.len(),
        duration
    );
}
