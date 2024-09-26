use core::fmt;

use bitvec::prelude::*;

/// polynomials for elements of GF(2) extension fields, coefficients are 0, 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    // coefficients are going to be 0,1 for binary fields
    bits: BitVec<u8, Lsb0>,
}

impl Polynomial {
    /// instantiate a polynomial using an array or a vector of 0s and 1s arranged in Big-endian
    /// order
    pub fn new(coeffs: &[u8]) -> Self {
        let mut bits = BitVec::<u8, Lsb0>::new();
        for &coeff in coeffs.iter().rev() {
            bits.push(coeff == 1);
        }
        while bits.len() > 1 && !bits.last().unwrap() {
            bits.pop();
        }
        Polynomial { bits }
    }

    // think about this and its use
    pub fn degree(&self) -> usize {
        self.bits.len().saturating_sub(1)
    }

    /// adds a polynomial to itself and returns the resultant
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut result = self.bits.clone();
        result.resize(usize::max(self.bits.len(), other.bits.len()), false);
        for i in 0..other.bits.len() {
            let bit = result[i];
            result.set(i, bit ^ other.bits[i]);
        }
        while result.len() > 1 && !result.last().unwrap() {
            result.pop();
        }
        Polynomial { bits: result }
    }

    // multiplies itself with another polynomial and returns the resultant
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result_bits = BitVec::<u8, Lsb0>::new();
        result_bits.resize(self.bits.len() + other.bits.len(), false);

        for i in 0..self.bits.len() {
            if self.bits[i] {
                for j in 0..other.bits.len() {
                    if other.bits[j] {
                        let idx = i + j;
                        if idx >= result_bits.len() {
                            result_bits.resize(idx + 1, false);
                        }
                        let bit = result_bits[idx];
                        result_bits.set(idx, bit ^ true);
                    }
                }
            }
        }

        while result_bits.len() > 1 && !result_bits.last().unwrap() {
            result_bits.pop();
        }

        Polynomial { bits: result_bits }
    }

    /// Divides self by other, returning the quotient and remainder.
    pub fn div_rem(&self, other: &Polynomial) -> (Polynomial, Polynomial) {
        let mut dividend = self.bits.clone();
        let divisor = &other.bits;
        let mut quotient_bits = BitVec::<u8, Lsb0>::new();

        while dividend.len() >= divisor.len() && dividend.iter().any(|b| *b) {
            let degree_diff = dividend.len() - divisor.len();

            // XOR the shifted divisor from the dividend
            for i in 0..divisor.len() {
                if divisor[i] {
                    let idx = i + degree_diff;
                    if idx < dividend.len() {
                        let var_name = dividend[idx];
                        dividend.set(idx, var_name ^ true);
                    }
                }
            }

            // Record the term in the quotient
            if degree_diff >= quotient_bits.len() {
                quotient_bits.resize(degree_diff + 1, false);
            }
            quotient_bits.set(degree_diff, true);

            // Remove leading zeros from dividend
            while dividend.len() > 1 && !dividend.last().unwrap() {
                dividend.pop();
            }
        }

        let quotient = Polynomial {
            bits: quotient_bits,
        };
        let remainder = Polynomial { bits: dividend };

        (quotient, remainder)
    }

    pub fn gcd(&self, other: &Polynomial) -> Polynomial {
        let mut a = self.clone();
        let mut b = other.clone();
        while b.bits.iter().any(|b| *b) {
            let (_, remainder) = a.div_rem(&b);
            a = b;
            b = remainder;
        }
        a
    }

    /// Compute the modular inverse of a polynomial given a modulus
    pub fn inverse(&self, modulus: &Polynomial) -> Option<Polynomial> {
        // Initialize r0 = modulus, r1 = self
        let mut r0 = modulus.clone();
        let mut r1 = self.clone();

        // Initialize s0 = 0, s1 = 1
        let mut s0 = Polynomial::new(&[0]); // Represents 0
        let mut s1 = Polynomial::new(&[1]); // Represents 1

        // Extended Euclidean Algorithm loop
        while r1.bits.iter().any(|b| *b) {
            let (q, _) = r0.div_rem(&r1); // Obtain quotient q

            // Compute r_new = r0 + q * r1 (since subtraction is XOR in GF(2))
            let r_new = r0.add(&q.multiply(&r1));
            r0 = r1;
            r1 = r_new;

            // Compute s_new = s0 + q * s1
            let s_new = s0.add(&q.multiply(&s1));
            s0 = s1;
            s1 = s_new;
        }

        // If r0 is 1, then inverse exists and is s0
        if r0.bits.len() == 1 && r0.bits[0] {
            Some(s0)
        } else {
            None // No inverse exists
        }
    }

    fn modulo(&self, modulus: &Polynomial) -> Polynomial {
        let mut remainder = self.bits.clone();
        let divisor = &modulus.bits;

        while remainder.len() >= divisor.len() && remainder.iter().any(|b| *b) {
            let degree_diff = remainder.len() - divisor.len();

            //XOR the shifted divisor from the remainder
            for i in 0..divisor.len() {
                if divisor[i] {
                    let idx = i + degree_diff;
                    if idx < remainder.len() {
                        let remainder_bit = remainder[idx];
                        remainder.set(idx, remainder_bit ^ true);
                    }
                }
            }
            while remainder.len() > 1 && !remainder.last().unwrap() {
                remainder.pop();
            }
        }

        Polynomial { bits: remainder }
    }

    fn x() -> Polynomial {
        Polynomial::new(&[1, 0])
    }

    /// Raises the polynomial x to the power of 2^k modulo the given modulus polynomial.
    fn pow2_mod(&self, k: usize, modulus: &Polynomial) -> Polynomial {
        let mut result = self.clone(); // Start with x (the current polynomial)
        for _ in 0..k {
            result = result.multiply(&result); // Square the polynomial
            result = result.modulo(modulus); // Reduce modulo f(x)
        }
        result
    }

    fn distinct_prime_factors(n: usize) -> Vec<usize> {
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

    /// Checks if the polynomial is irreducible over GF(2) using Rabin's Test
    pub fn is_irreducible(&self) -> bool {
        let n = self.degree();
        if n <= 0 {
            return false; // Degree must be at least 1
        }
        if n == 1 {
            return true; // All degree 1 polynomials are irreducible
        }

        let prime_factors = Polynomial::distinct_prime_factors(n);
        let x = Polynomial::x();

        for &p in &prime_factors {
            let exponent = n / p;
            let x_exp = x.pow2_mod(exponent, self);
            let diff = x_exp.add(&x); // x^{2^{n/p}} - x == x^{2^{n/p}} + x in GF(2)
            let gcd = self.gcd(&diff);
            if gcd.degree() >= 1 {
                return false; // Reducible
            }
        }

        // Finally, check that x^{2^n} mod f(x) == x
        let x_final = x.pow2_mod(n, self);
        let condition = x_final.add(&x) == Polynomial::new(&[0]); // Should equal 0
        condition
    }

    /// Converts the polynomial into a human-readable format
    pub fn to_string(&self) -> String {
        let mut terms = Vec::new();

        for (i, bit) in self.bits.iter().enumerate() {
            if *bit {
                let current_degree = i;
                let term = match current_degree {
                    0 => "1".to_string(),
                    1 => "x".to_string(),
                    _ => format!("x^{}", current_degree),
                };
                terms.push(term);
            }
        }

        if terms.is_empty() {
            "0".to_string()
        } else {
            // Reverse the terms to display highest degree first
            let reversed_terms: Vec<String> = terms.into_iter().rev().collect();
            reversed_terms.join(" + ")
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let poly_str = self.to_string();
        write!(f, "{}", poly_str)
    }
}
