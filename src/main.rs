fn gcd(a: &i128, b: &i128) -> i128 {
    let mut a = *a;
    let mut b = *b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn gcd_recursive(a: &i128, b: &i128) -> i128 {
    if *b == 0 {
        *a
    } else {
        gcd_recursive(b, &(a % b))
    }
}

fn modular_exponentiation(base: &i128, exponent: &i128, modulus: &i128) -> i128 {
    let mut result = 1i128;
    let mut base_mod = *base % *modulus;
    let mut exp = *exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base_mod) % *modulus;
        }
        base_mod = (base_mod * base_mod) % *modulus;
        exp >>= 1;
    }
    result
}

fn extended_gcd(a: &i128, b: &i128) -> (i128, i128, i128) {
    if *b == 0 {
        return (*a, 1, 0);
    }

    let (gcd, x1, y1) = extended_gcd(b, &(*a % *b));
    let x = y1;
    let y = x1 - (*a / *b) * y1;

    (gcd, x, y)
}

fn modular_inverse(a: &i128, m: &i128) -> Option<i128> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        return None;
    }
    Some((x % m + m) % m)
}

fn segment_sieve(n: usize) -> Vec<usize> {
    let segment_size = 1_000_000; // Adjust based on memory
    let limit = (n as f64).sqrt().ceil() as usize;

    // Step 1: Sieve up to sqrt(n) (only odd numbers)
    let mut is_prime = vec![true; (limit / 2) + 1]; // +1 to handle odd indices
    let mut primes_sqrt_n = vec![2]; // Start with 2, the only even prime

    for i in 1..is_prime.len() {
        let p = 2 * i + 1; // Convert index to odd number (2i + 1)
        if is_prime[i] {
            primes_sqrt_n.push(p);
            // Mark multiples of p starting from p^2, but only odd multiples
            for multiple in ((p * p)..=limit).step_by(2 * p) {
                is_prime[(multiple - 1) / 2] = false; // Mark odd multiples
            }
        }
    }

    // Step 2: Segmented sieve for the range [sqrt(n), n]
    let mut primes_in_range = Vec::new(); // To store primes found in segments
    let mut low = limit + 1;
    let mut high = std::cmp::min(low + segment_size - 1, n);

    while low <= n {
        // Boolean vector for the current segment (only odd numbers)
        let mut is_prime_segment = vec![true; (segment_size / 2) + 1]; // +1 to handle last odd number

        // Mark multiples of primes from sqrt(n) in the current segment
        for &p in &primes_sqrt_n {
            // Find the first multiple of p within the current segment
            let mut start = if p * p > low {
                p * p // Start from p^2 if it's within the segment
            } else {
                ((low + p - 1) / p) * p // Start from the smallest multiple of p within the segment
            };

            // Adjust start to be odd if it's even
            if start % 2 == 0 {
                start += p;
            }

            // Mark odd multiples of p in the segment
            while start <= high {
                if start % 2 == 1 {
                    is_prime_segment[(start - low) / 2] = false;
                }
                start += 2 * p; // Skip even multiples
            }
        }

        // Collect primes from the current segment
        for i in (low..=high).step_by(2) {
            if is_prime_segment[(i - low) / 2] {
                primes_in_range.push(i); // Collect the prime
            }
        }

        // Move to the next segment
        low = high + 1;
        high = std::cmp::min(low + segment_size - 1, n);
    }

    // Combine primes up to sqrt(n) with primes from segmented sieve
    primes_sqrt_n.extend(primes_in_range);

    primes_sqrt_n // Return all primes found up to n
}

fn main() {
    let n = 100_000_000;
    let primes = segment_sieve(n);
    println!("Primes: {:?}", &primes[..10000]);
}
