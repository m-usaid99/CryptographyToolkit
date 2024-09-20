pub fn segment_sieve(n: usize) -> Vec<usize> {
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

pub fn segment_sieve_wheel(n: usize) -> Vec<usize> {
    let wheel_pattern = vec![1, 7, 11, 13, 17, 19, 23, 29]; // Wheel pattern for 2, 3, 5
    let wheel_size = 30; // Product of 2, 3, 5

    let segment_size = 8_000_000; // Adjust based on memory
    let limit = (n as f64).sqrt().ceil() as usize;

    // Step 1: Simple sieve for small primes up to sqrt(n)
    let mut is_prime = vec![true; (limit / 2) + 1];
    let mut primes = vec![2, 3, 5]; // Start with small primes

    for i in 1..is_prime.len() {
        let num = 2 * i + 1;
        if is_prime[i] {
            primes.push(num);
            for multiple in ((num * num)..=limit).step_by(2 * num) {
                is_prime[(multiple - 1) / 2] = false;
            }
        }
    }

    // Step 2: Segmented sieve using the wheel
    let mut low = limit + 1;
    let mut high = std::cmp::min(low + segment_size - 1, n);
    let mut primes_in_range = Vec::new();

    while low <= n {
        // Boolean vector for the current segment (only odd numbers)
        let mut is_prime_segment = vec![true; (segment_size / 2) + 1];

        // Mark multiples of primes from small primes in the current segment
        for &p in &primes {
            let mut start = if p * p > low {
                p * p // Start from p^2 if it's within the segment
            } else {
                ((low + p - 1) / p) * p // Start from the smallest multiple of p within the segment
            };

            if start % 2 == 0 {
                start += p; // Ensure start is odd
            }

            // Mark odd multiples of p in the segment
            while start <= high {
                is_prime_segment[(start - low) / 2] = false;
                start += 2 * p; // Skip even multiples
            }
        }

        // Collect primes from the current segment using the wheel pattern
        for i in (low..=high).step_by(2) {
            if is_prime_segment[(i - low) / 2] {
                let mod_result = i % wheel_size;
                if wheel_pattern.contains(&mod_result) {
                    primes_in_range.push(i); // Collect primes fitting the wheel pattern
                }
            }
        }

        // Move to the next segment
        low = high + 1;
        high = std::cmp::min(low + segment_size - 1, n);
    }

    primes.extend(primes_in_range); // Combine small primes with those from the segmented sieve
    primes
}
