// src/polynomial/utils.rs

use super::Polynomial;

impl Polynomial {
    /// Returns the degree of the polynomial.
    pub fn degree(&self) -> usize {
        self.bits.len().saturating_sub(1)
    }

    /// Checks if the polynomial is zero.
    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|b| !b)
    }

    pub fn shift_left(&self, n: usize) -> Polynomial {
        let mut shifted_bits = self.bits.clone();

        for _ in 0..n {
            shifted_bits.push(false); // Append zeros at the end
        }

        Polynomial { bits: shifted_bits }
    }

    pub fn x() -> Polynomial {
        Polynomial::new(&[1, 0])
    }

    pub fn pow2_mod(&self, k: usize, modulus: &Polynomial) -> Polynomial {
        let mut result = self.clone();
        for _ in 0..k {
            result = result.square().modulo(modulus);
        }
        result
    }

    pub fn distinct_prime_factors(n: usize) -> Vec<usize> {
        let mut factors = Vec::new();
        let mut num = n;
        if num % 2 == 0 {
            factors.push(2);
            while num % 2 == 0 {
                num /= 2;
            }
        }
        let mut i = 3;
        while i * i <= num {
            if num % i == 0 {
                factors.push(i);
                while num % i == 0 {
                    num /= i;
                }
            }
            i += 2;
        }
        if num > 2 {
            factors.push(num);
        }
        factors
    }

    /// Returns true if the polynomial is equal to one.
    pub fn is_one(&self) -> bool {
        self.bits.len() == 1 && self.bits[0]
    }

    pub fn one() -> Polynomial {
        Polynomial::new(&[1])
    }

    /// enumerate all possible monics of a certain degree
    pub fn generate_all_monics(degree: usize) -> Vec<Polynomial> {
        let mut polynomials = Vec::new();
        let total = 1 << degree;

        for i in 0..total {
            let mut coeffs = vec![0u8; degree + 1];
            coeffs[degree] = 1; // ensure its a monic
            for j in 0..degree {
                coeffs[j] = ((i >> j) & 1) as u8;
            }
            polynomials.push(Polynomial::new(&coeffs));
        }
        polynomials
    }
}
