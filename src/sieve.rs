use rayon::prelude::*;
use std::sync::Mutex;
use std::thread; // To prevent logs from mixing up

/// Implements the Segmented Sieve of Eratosthenes with Wheel Factorization (2-wheel) to find all primes up to `n`.
/// Returns a vector of prime numbers.
pub fn segmented_sieve_with_wheel(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    // Step 1: Handle the first prime separately
    let mut primes = vec![2];
    if n == 2 {
        return primes;
    }

    // Step 2: Find all primes up to sqrt(n) using the simple sieve with wheel (skip even numbers)
    let limit = (n as f64).sqrt() as usize;
    let sieve_size = (limit / 2) + 1; // Only odd numbers
    let mut is_prime_small = vec![true; sieve_size];
    is_prime_small[0] = false; // 1 is not prime

    for i in 1..sieve_size {
        if is_prime_small[i] {
            let prime = 2 * i + 1;
            let start = (prime * prime) / 2;
            for multiple in (start..sieve_size).step_by(prime) {
                is_prime_small[multiple] = false;
            }
        }
    }

    // Collect primes up to sqrt(n)
    let small_primes: Vec<usize> = is_prime_small
        .iter()
        .enumerate()
        .filter_map(|(i, &is_p)| if is_p { Some(2 * i + 1) } else { None })
        .collect();

    primes.extend(&small_primes);

    // Step 3: Initialize variables for segmented sieve
    let segment_size = 500000; // Adjust based on memory
    let mut low = limit + 1;
    if low % 2 == 0 {
        low += 1;
    }
    let mut high = low + segment_size;
    if high > n {
        high = n + 1;
    }

    // Step 4: Process each segment
    while low <= n {
        let current_size = (high - low + 1) / 2; // Only odd numbers
        let mut is_prime = vec![true; current_size];

        for &prime in &small_primes {
            // Optimization 3: Skip primes below the segment
            // Find the minimum multiple of prime >= low
            let mut start = if prime * prime >= low {
                prime * prime // Start marking from prime * prime
            } else {
                // Start from the first multiple of prime within [low, high)
                let remainder = low % prime;
                if remainder == 0 {
                    low
                } else {
                    low + (prime - remainder)
                }
            };

            // Ensure that 'start' is an odd number
            if start % 2 == 0 {
                start += prime;
            }

            // Mark multiples of prime within the segment
            for multiple in (start..high).step_by(prime * 2) {
                // step_by prime*2 to stay with odd multiples
                let index = (multiple - low) / 2;
                if index < is_prime.len() {
                    is_prime[index] = false;
                }
            }
        }

        // Collect primes in the current segment
        for i in 0..current_size {
            if is_prime[i] {
                let num = low + 2 * i;
                if num <= n {
                    primes.push(num);
                }
            }
        }

        // Move to the next segment
        low += segment_size;
        if low % 2 == 0 {
            low += 1;
        }
        high += segment_size;
        if high > n + 1 {
            high = n + 1;
        }
    }

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

fn set_bit(bits: &mut Vec<u64>, index: usize) {
    bits[index / 64] |= 1 << (index % 64);
}

fn is_bit_set(bits: &Vec<u64>, index: usize) -> bool {
    bits[index / 64] & (1 << (index % 64)) != 0
}

/// Parallelized segmented sieve with bitset and logging.
pub fn parallel_segmented_sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    // Find all primes up to sqrt(n) using a simple sieve.
    let limit = (n as f64).sqrt() as usize;
    let sieve_size = (limit / 2) + 1; // Only odd numbers
    let mut is_prime_small = vec![0u64; (sieve_size + 63) / 64]; // Bitset for small sieve

    // Simple sieve for primes up to sqrt(n)
    for i in 1..sieve_size {
        if !is_bit_set(&is_prime_small, i) {
            let prime = 2 * i + 1;
            let start = (prime * prime) / 2;
            for multiple in (start..sieve_size).step_by(prime) {
                set_bit(&mut is_prime_small, multiple);
            }
        }
    }

    // Collect small primes up to sqrt(n)
    let small_primes: Vec<usize> = (1..sieve_size)
        .filter(|&i| !is_bit_set(&is_prime_small, i))
        .map(|i| 2 * i + 1)
        .collect();

    let mut primes = vec![2];
    primes.extend(&small_primes);

    // Define segment size (based on L2 cache size)
    let segment_size = 4 * 1024 * 1024 * 8 / 2; // 4MB cache-based segment size

    // Mutex to synchronize logging output
    let log_mutex = Mutex::new(());

    // Parallelize the segmented sieve using Rayon with bitset and logging.
    let num_segments = (n - limit) / (2 * segment_size) + 1;
    let segments: Vec<Vec<usize>> = (0..num_segments)
        .into_par_iter()
        .map(|segment_idx| {
            let mut local_primes = Vec::new();
            let low = limit + 1 + 2 * segment_idx * segment_size;
            let mut high = low + 2 * segment_size;
            if high > n {
                high = n + 1;
            }

            // Get current thread ID
            let thread_id = format!("{:?}", thread::current().id());
            // Log the segment information
            {
                let _lock = log_mutex.lock().unwrap();
                println!(
                    "Thread {} is processing segment {}: low = {}, high = {}",
                    thread_id, segment_idx, low, high
                );
            }

            let current_size = (high - low + 1) / 2; // Only odd numbers
            let mut is_prime_segment = vec![0u64; (current_size + 63) / 64]; // Bitset for the segment

            // Mark multiples of small primes in the segment
            for &prime in &small_primes {
                let mut start = if prime * prime >= low {
                    prime * prime
                } else {
                    let remainder = low % prime;
                    if remainder == 0 {
                        low
                    } else {
                        low + (prime - remainder)
                    }
                };

                // Ensure start is odd
                if start % 2 == 0 {
                    start += prime;
                }

                for multiple in (start..high).step_by(prime * 2) {
                    let index = (multiple - low) / 2;
                    if index < current_size {
                        set_bit(&mut is_prime_segment, index);
                    }
                }
            }

            // Collect primes in the current segment
            for i in 0..current_size {
                if !is_bit_set(&is_prime_segment, i) {
                    let num = low + 2 * i;
                    if num <= n {
                        local_primes.push(num);
                    }
                }
            }

            local_primes
        })
        .collect();

    // Flatten the results from each segment into a single vector
    segments
        .into_iter()
        .for_each(|segment| primes.extend(segment));

    primes
}
