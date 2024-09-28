use super::Polynomial;
use bitvec::prelude::*;

impl Polynomial {
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

        // Extended Euclidean Algorithm loop
        while !r1.is_zero() {
            // Perform division: r0 = q * r1 + remainder
            let (q, r) = r0.div_rem(&r1);

            // Update r0 and r1
            r0 = r1;
            r1 = r;

            // Update s_new = s0 + q * s1
            let s_new = s0.add(&q.multiply(&s1)).modulo(modulus);
            s0 = s1;
            s1 = s_new;

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
}
