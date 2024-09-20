mod mod_arith;
mod sieve;

fn main() {
    let n = 1_000_000_000;
    //let primes = sieve::segment_sieve(n);
    let start = std::time::Instant::now();
    let primes = sieve::segment_sieve_wheel(n);
    let duration = start.elapsed();
    println!("Segment size: {} numbers, Time: {:?}", 16_000_000, duration);
}
