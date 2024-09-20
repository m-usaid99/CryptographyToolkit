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

pub fn sieve_of_eratosthenes_bitset(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut primes = vec![2];
    let bitset_size = (n + 1) / 2; // only store odd numberrs
    let mut is_composite = vec![0u64; (bitset_size + 63) / 64];

    // Step 2: Mark multiples of primes starting from 2
    let limit = (n as f64).sqrt().ceil() as usize;
    for p in (3..=limit).step_by(2) {
        // If 'p' is still marked as prime (its bit is 0), mark its multiples as composite
        if (is_composite[p / 2 / 64] & (1 << ((p / 2) % 64))) == 0 {
            // Mark all multiples of 'p' starting from p^2
            let mut multiple = p * p;
            while multiple <= n {
                is_composite[multiple / 2 / 64] |= 1 << ((multiple / 2) % 64); // Set bit (mark as composite)
                multiple += 2 * p;
            }
        }
    }

    for num in (3..=n).step_by(2) {
        if (is_composite[num / 2 / 64] & (1 << ((num / 2) % 64))) == 0 {
            primes.push(num);
        }
    }

    primes
}

pub fn segmented_sieve_bitset(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    // Step 1: Use a regular sieve to precompute primes up to sqrt(n)
    let limit = (n as f64).sqrt().ceil() as usize;
    let mut base_primes = vec![2];
    let base_bitset_size = (limit + 1) / 2; // Store only odd numbers
    let mut is_base_composite = vec![0u64; (base_bitset_size + 63) / 64];

    // Mark multiples of small primes in the base sieve
    for p in (3..=limit).step_by(2) {
        if (is_base_composite[p / 2 / 64] & (1 << ((p / 2) % 64))) == 0 {
            // Add p to the list of base primes
            base_primes.push(p);

            // Mark multiples of p in the base sieve
            let mut multiple = p * p;
            while multiple <= limit {
                is_base_composite[multiple / 2 / 64] |= 1 << ((multiple / 2) % 64);
                multiple += 2 * p;
            }
        }
    }

    let segment_size = 3_000_000;
    let mut primes = base_primes.clone();

    let mut low = limit + 1;
    let mut high = std::cmp::min(low + segment_size - 1, n);

    while low <= n {
        let mut is_composite_segment = vec![0u64; (segment_size + 1) / 2];
        for &p in &base_primes {
            let mut start = if p * p > low {
                p * p
            } else {
                ((low + p - 1) / p) * p
            };

            if start % 2 == 0 {
                start += p;
            }

            while start <= high {
                if start > low {
                    is_composite_segment[(start - low) / 2 / 64] |= 1 << ((start - low) / 2 % 64);
                }
                start += 2 * p;
            }
        }
        for i in (low..=high).step_by(2) {
            if (is_composite_segment[(i - low) / 2 / 64] & (1 << ((i - low) / 2 % 64))) == 0 {
                primes.push(i);
            }
        }

        low = high + 1;
        high = std::cmp::min(low + segment_size - 1, n);
    }

    primes
}
