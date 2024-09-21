mod mod_arith;
mod sieve;

fn main() {
    //let n = 10_000_000_000;
    let start = std::time::Instant::now();
    //let primes = sieve::parallel_segmented_sieve(n);
    let base: i128 = 123456789012345678901234567890123456789;
    let exponent: i128 = 9876543210987654321;
    let modulus: i128 = 170141183460469231731687303715884105727;
    let out = mod_arith::modular_exponentiation(&base, &exponent, &modulus);
    let duration = start.elapsed();
    //println!(
    //    "Running for n: {}\nTotal primes: {:?} numbers, Time: {:?}",
    //    n,
    //    primes.len(),
    //    duration
    //);
    //
    println!(
        "Running modular exponentiation for base:{}, exponent:{}, and modulus:{}\n
        Output: {}\n
        Operation took {:?}",
        base, exponent, modulus, out, duration
    );
}
