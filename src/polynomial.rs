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
        let max_len = usize::max(self.bits.len(), other.bits.len());

        let self_len = self.bits.len();
        let other_len = other.bits.len();

        // Resize `self.bits` to the maximum length
        self.bits.resize(max_len, false);

        // Create a new BitVec to hold the result
        let mut result_bits = BitVec::<u8, Msb0>::with_capacity(max_len);

        // Iterate over the bits from most significant to least significant
        for i in 0..max_len {
            let self_idx = if i >= max_len - self_len {
                Some(i - (max_len - self_len))
            } else {
                None
            };

            let other_idx = if i >= max_len - other_len {
                Some(i - (max_len - other_len))
            } else {
                None
            };

            let self_bit = self_idx
                .and_then(|idx| self.bits.get(idx))
                .map(|b| *b)
                .unwrap_or(false);
            let other_bit = other_idx
                .and_then(|idx| other.bits.get(idx))
                .map(|b| *b)
                .unwrap_or(false);

            result_bits.push(self_bit ^ other_bit);
        }

        // Update self.bits with the result
        self.bits = result_bits;

        // Trim leading zeros
        while self.bits.len() > 1 && !self.bits.first().unwrap() {
            self.bits.remove(0);
        }
    }

    // multiplies itself with another polynomial and returns the resultant
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let result_len = self.bits.len() + other.bits.len() - 1;
        let mut result_bits = BitVec::<u8, Msb0>::repeat(false, result_len);

        for i in 0..self.bits.len() {
            if self.bits[i] {
                for j in 0..other.bits.len() {
                    if other.bits[j] {
                        let idx = i + j;
                        let bit = result_bits[idx];
                        result_bits.set(idx, bit ^ true);
                    }
                }
            }
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        Polynomial { bits: result_bits }
    }

    /// Multiplies `self` with another polynomial in place.
    pub fn multiply_in_place(&mut self, other: &Polynomial) {
        let self_len = self.bits.len();
        let other_len = other.bits.len();
        let result_len = self_len + other_len - 1;
        let mut result_bits = bitvec![u8, Msb0; 0; result_len];

        for i in 0..self_len {
            if self.bits[i] {
                for j in 0..other_len {
                    if other.bits[j] {
                        let idx = i + j;

                        // Perform XOR
                        let result_bit = result_bits[idx];
                        result_bits.set(idx, result_bit ^ true);
                    }
                }
            }
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        // Update self.bits
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
        // Clone the dividend and divisor
        let mut dividend = self.clone();
        let divisor = other.clone();

        // Handle division by zero
        if divisor.is_zero() {
            panic!("Cannot divide by zero polynomial");
        }

        // Calculate the maximum degree difference
        let max_diff = if dividend.degree() >= divisor.degree() {
            dividend.degree() - divisor.degree()
        } else {
            0
        };

        // Initialize quotient_bits with all bits set to false
        let mut quotient_bits = BitVec::<u8, Msb0>::repeat(false, max_diff + 1);

        // Loop until the dividend's degree is less than the divisor's degree
        while dividend.degree() >= divisor.degree() && !dividend.is_zero() {
            let degree_diff = dividend.degree() - divisor.degree();

            // Set the corresponding bit in the quotient
            // In Msb0, the highest degree bit is at index 0
            let quotient_idx = max_diff - degree_diff;
            quotient_bits.set(quotient_idx, true);

            // Shift the divisor left by degree_diff
            let shifted_divisor = divisor.shift_left(degree_diff);

            // Subtract (XOR in GF(2))
            dividend = dividend.add(&shifted_divisor);

            // Trim leading zeros to update the dividend's degree
            while dividend.bits.len() > 1 && !dividend.bits.first().unwrap() {
                dividend.bits.remove(0);
            }
        }

        // Create the quotient Polynomial
        let quotient = Polynomial {
            bits: quotient_bits,
        };

        // The remainder is the current dividend
        let remainder = dividend;

        (quotient, remainder)
    }

    /// Creates a polynomial representing a single term x^degree
    fn single_term(degree: usize) -> Polynomial {
        let mut bits = BitVec::<u8, Msb0>::repeat(false, degree + 1);
        bits.set(0, true); // Set the bit corresponding to x^degree
        Polynomial { bits }
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
        // Clone the initial polynomials
        let mut r0 = self.clone(); // Initialize r0 with 'self'
        let mut r1 = modulus.clone(); // Initialize r1 with 'modulus'

        // Initialize s0 and s1 as per standard EEA
        let mut s0 = Polynomial::new(&[1]); // Represents 1
        let mut s1 = Polynomial::new(&[0]); // Represents 0

        // Debugging: Initial state
        println!("Initial State:");
        println!("r0: {}", r0);
        println!("r1: {}", r1);
        println!("s0: {}", s0);
        println!("s1: {}", s1);
        println!("-----------------------------------");

        // Extended Euclidean Algorithm loop
        while !r1.is_zero() {
            // Perform division: r0 = q * r1 + remainder
            let (q, r) = r0.div_rem(&r1);
            println!("Quotient: {}", q);
            println!("Remainder: {}", r);

            // Update r0 and r1
            r0 = r1;
            r1 = r;

            // Update s_new = s0 + q * s1
            let s_new = s0.add(&q.multiply(&s1)).modulo(modulus);
            s0 = s1;
            s1 = s_new;

            // Debugging: State after each iteration
            println!("After Iteration:");
            println!("r0: {}", r0);
            println!("r1: {}", r1);
            println!("s0: {}", s0);
            println!("s1: {}", s1);
            println!("-----------------------------------");

            // Safety Check: Prevent infinite loops
            if r1.degree() > r0.degree() {
                println!("Detected non-reducing r1. Breaking to prevent infinite loop.");
                break;
            }
        }

        // Final check: If r0 is 1, inverse exists
        if r0.bits.len() == 1 && r0.bits[0] {
            // Ensure the inverse is reduced modulo the modulus
            Some(s0.modulo(modulus))
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
