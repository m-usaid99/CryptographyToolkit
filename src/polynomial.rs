use bitvec::prelude::*;
use core::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// polynomials for elements of GF(2) extension fields, coefficients are 0, 1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    // coefficients are going to be 0,1 for binary fields
    bits: BitVec<u8, Msb0>,
}

impl Polynomial {
    /// instantiate a polynomial using an array or a vector of 0s and 1s arranged in Big-endian
    /// order
    pub fn new(coeffs: &[u8]) -> Self {
        let mut bits = BitVec::<u8, Msb0>::new();
        for &coeff in coeffs.iter() {
            bits.push(coeff == 1);
        }
        // Trim leading zeros from the beginning
        while bits.len() > 1 && !bits.first().unwrap() {
            bits.remove(0);
        }
        Polynomial { bits }
    }

    // think about this and its use
    pub fn degree(&self) -> usize {
        self.bits.len().saturating_sub(1)
    }

    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|b| !b)
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = usize::max(self.bits.len(), other.bits.len());
        let mut result_bits = BitVec::<u8, Msb0>::with_capacity(max_len);

        let self_offset = max_len - self.bits.len();
        let other_offset = max_len - other.bits.len();

        for i in 0..max_len {
            let self_bit = if i >= self_offset {
                self.bits[i - self_offset]
            } else {
                false
            };

            let other_bit = if i >= other_offset {
                other.bits[i - other_offset]
            } else {
                false
            };

            result_bits.push(self_bit ^ other_bit);
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        Polynomial { bits: result_bits }
    }

    /// Adds another polynomial to `self` in place.
    pub fn add_in_place(&mut self, other: &Polynomial) {
        // Resize `self.bits` to the maximum length of both polynomials
        self.bits
            .resize(usize::max(self.bits.len(), other.bits.len()), false);

        // Perform XOR for addition in GF(2)
        for i in 0..other.bits.len() {
            let bit = self.bits[i];
            self.bits.set(i, bit ^ other.bits[i]);
        }

        // Trim leading zeros to maintain the correct degree
        self.trim_leading_zeros();
    }

    /// Trims leading zeros from `self.bits`
    fn trim_leading_zeros(&mut self) {
        while self.bits.len() > 1 && !self.bits.last().unwrap() {
            self.bits.pop();
        }
    }

    // multiplies itself with another polynomial and returns the resultant
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result_bits = BitVec::<u8, Msb0>::new();
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

    /// Multiplies `self` with another polynomial in place.
    pub fn multiply_in_place(&mut self, other: &Polynomial) {
        let mut result_bits = BitVec::<u8, Msb0>::new();
        result_bits.resize(self.bits.len() + other.bits.len(), false);

        for i in 0..self.bits.len() {
            if self.bits[i] {
                for j in 0..other.bits.len() {
                    if other.bits[j] {
                        let idx = i + j;
                        // Ensure the result_bits can accommodate the index
                        if idx >= result_bits.len() {
                            result_bits.resize(idx + 1, false);
                        }
                        // Perform XOR for addition in GF(2)
                        let bit = result_bits[idx];
                        result_bits.set(idx, bit ^ true);
                    }
                }
            }
        }

        // Trim leading zeros to maintain the correct degree
        while result_bits.len() > 1 && !result_bits.last().unwrap() {
            result_bits.pop();
        }

        // Update `self.bits` with the result
        self.bits = result_bits;
    }

    pub fn square(&self) -> Polynomial {
        let mut squared_bits = BitVec::<u8, Msb0>::with_capacity(self.bits.len() * 2);

        for bit in self.bits.iter() {
            squared_bits.push(*bit);
            squared_bits.push(false); // Insert a zero after each bit
        }

        // Remove trailing zero if added extra
        if squared_bits.len() > 0 && !squared_bits.last().unwrap() {
            squared_bits.pop();
        }

        // Trim leading zeros
        while squared_bits.len() > 1 && !squared_bits.first().unwrap() {
            squared_bits.remove(0);
        }

        Polynomial { bits: squared_bits }
    }

    /// Divides self by other, returning the quotient and remainder.
    pub fn div_rem(&self, other: &Polynomial) -> (Polynomial, Polynomial) {
        let mut dividend = self.bits.clone();
        let divisor = &other.bits;
        let mut quotient_bits = BitVec::<u8, Msb0>::new();

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
        while !b.is_zero() {
            let remainder = a.modulo(&b);
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

    pub fn shift_left(&self, n: usize) -> Polynomial {
        let mut shifted_bits = self.bits.clone();

        for _ in 0..n {
            shifted_bits.push(false); // Append zeros at the end
        }

        Polynomial { bits: shifted_bits }
    }

    pub fn modulo(&self, modulus: &Polynomial) -> Polynomial {
        let mut remainder = self.clone();

        while remainder.degree() >= modulus.degree() && !remainder.is_zero() {
            let degree_diff = remainder.degree() - modulus.degree();
            let shifted_modulus = modulus.shift_left(degree_diff);

            // Subtract (XOR in GF(2))
            remainder = remainder.add(&shifted_modulus);

            // Trim leading zeros
            while remainder.bits.len() > 1 && !remainder.bits.first().unwrap() {
                remainder.bits.remove(0);
            }
        }

        remainder
    }

    fn x() -> Polynomial {
        Polynomial::new(&[1, 0])
    }

    fn pow2_mod(&self, k: usize, modulus: &Polynomial) -> Polynomial {
        let mut result = self.clone();
        for _ in 0..k {
            result = result.square().modulo(modulus);
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

    // generate all possible trinomials
    pub fn generate_trinomials(degree: usize) -> Vec<Polynomial> {
        let mut trinomials = Vec::new();
        for k in (1..degree).rev() {
            let mut coeffs = vec![0u8; degree + 1];
            coeffs[degree] = 1;
            coeffs[k] = 1;
            coeffs[0] = 1;
            trinomials.push(Polynomial::new(&coeffs));
        }
        trinomials
    }

    /// returns an irreducible trinomial if exists
    pub fn irreducible_trinomial(degree: usize) -> Option<Polynomial> {
        let tris = Polynomial::generate_trinomials(degree);
        println!("{:?}", tris);
        for trinomial in tris {
            if trinomial.is_irreducible() {
                return Some(trinomial);
            }
        }
        None
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

    /// generate an irreducible monic of degree `n` to serve as modulus for finite field
    pub fn irreducible_element(degree: usize) -> Option<Polynomial> {
        // try to find irreducible trinomial
        if let Some(tri_poly) = Polynomial::irreducible_trinomial(degree) {
            return Some(tri_poly);
        }

        // if no irreducible trinomial, enumerate all possible monics
        for poly in Polynomial::generate_all_monics(degree) {
            if poly.is_irreducible() {
                return Some(poly);
            }
        }
        None
    }

    /// Converts the polynomial into a human-readable format
    pub fn to_string(&self) -> String {
        let mut terms = Vec::new();
        let degree = self.degree();

        for (i, bit) in self.bits.iter().enumerate() {
            if *bit {
                let current_degree = degree - i;
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
            terms.join(" + ")
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let poly_str = self.to_string();
        write!(f, "{}", poly_str)
    }
}

impl<'a, 'b> Add<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;
    fn add(self, other: &'b Polynomial) -> Polynomial {
        self.add(other)
    }
}

impl<'a, 'b> Mul<&'b Polynomial> for &'a Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &'b Polynomial) -> Polynomial {
        self.multiply(other)
    }
}

impl<'a> AddAssign<&'a Polynomial> for Polynomial {
    fn add_assign(&mut self, other: &'a Polynomial) {
        *self = self.add(other);
    }
}

impl<'a> MulAssign<&'a Polynomial> for Polynomial {
    fn mul_assign(&mut self, other: &'a Polynomial) {
        *self = self.multiply(other);
    }
}
