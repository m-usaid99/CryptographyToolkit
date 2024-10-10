// src/integer_mod_n/integer_mod_n.rs

use crate::algebra::traits::{Algebra, Group, Ring};
use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_integer::Integer; // Import the Integer trait
use num_traits::{One, Zero};
use rand::rngs::OsRng;
use rand::Rng;
use std::fmt;

/// Errors related to `IntegerModN`.
#[derive(Debug)]
pub enum IntegerModNError {
    InversionFailed,
}

impl fmt::Display for IntegerModNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegerModNError::InversionFailed => {
                write!(f, "Multiplicative inverse does not exist.")
            }
        }
    }
}

impl std::error::Error for IntegerModNError {}

/// Represents an integer modulo a composite number `n`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerModN {
    n: BigUint, // Composite modulus
}

impl IntegerModN {
    /// Creates a new `IntegerModN` with a given modulus `n`.
    pub fn new(n: BigUint) -> Self {
        IntegerModN { n }
    }

    /// Computes the multiplicative inverse using the Extended Euclidean Algorithm.
    pub fn inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
        let (gcd, x, _) = Self::extended_gcd(a, n);
        if gcd != BigUint::one() {
            return None; // Inverse does not exist
        }

        // Compute x mod n to ensure the inverse is positive
        let n_bigint = n.to_bigint().unwrap();
        let x_mod_n = x.mod_floor(&n_bigint).to_biguint().unwrap();
        Some(x_mod_n)
    }

    /// Extended Euclidean Algorithm.
    /// Returns a tuple of (gcd, x, y) such that ax + by = gcd(a, b)
    fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
        let a_bigint = a.to_bigint().unwrap();
        let b_bigint = b.to_bigint().unwrap();

        let mut old_r = a_bigint.clone();
        let mut r = b_bigint.clone();
        let mut old_s = BigInt::one();
        let mut s = BigInt::zero();
        let mut old_t = BigInt::zero();
        let mut t = BigInt::one();

        while !r.is_zero() {
            let quotient = &old_r / &r;
            let temp_r = r.clone();
            r = &old_r - &quotient * &r;
            old_r = temp_r;

            let temp_s = s.clone();
            s = &old_s - &quotient * &s;
            old_s = temp_s;

            let temp_t = t.clone();
            t = &old_t - &quotient * &t;
            old_t = temp_t;
        }

        // Convert gcd back to BigUint
        let gcd = old_r.to_biguint().unwrap();

        (gcd, old_s, old_t)
    }
    /// Raises `base` to the power `exp` modulo `n` using exponentiation by squaring.
    pub fn pow(base: &BigUint, exp: &BigUint, n: &BigUint) -> BigUint {
        base.modpow(&exp, n)
    }

    /// Generates a random element in the multiplicative group (Z/nZ)*.
    pub fn random_group_element(&self) -> BigUint {
        loop {
            let random_num = self.random_element();
            if Self::gcd(&random_num, &self.n) == BigUint::one() {
                return random_num;
            }
            // Retry if not coprime
        }
    }

    /// Helper method to generate a random element in Z/nZ.
    fn random_element(&self) -> BigUint {
        let mut rng = OsRng;
        let bytes = self.n.to_bytes_be();
        let mut random_bytes = vec![0u8; bytes.len()];
        rng.fill(&mut random_bytes[..]);
        let random_num = BigUint::from_bytes_be(&random_bytes);
        &random_num % &self.n
    }

    pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
        let mut a = a.clone();
        let mut b = b.clone();

        while !b.is_zero() {
            let temp = b.clone();
            b = &a % &b;
            a = temp;
        }

        a
    }

    /// Checks if a given number is an element of Z/nZ.
    /// Accepts any type that can be converted into a `BigUint`.
    pub fn contains<T: Into<BigUint>>(&self, num: T) -> bool {
        let num = num.into();
        num < self.n
    }
}

impl Algebra for IntegerModN {
    type Element = BigUint;
}

impl Ring for IntegerModN {
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        (a + b) % &self.n
    }

    fn mul(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        (a * b) % &self.n
    }

    fn zero(&self) -> Self::Element {
        BigUint::zero()
    }

    fn one(&self) -> Self::Element {
        BigUint::one()
    }
}

impl Group for IntegerModN {
    fn combine(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        self.mul(a, b)
    }

    fn identity(&self) -> Self::Element {
        self.one()
    }

    fn inverse(&self, a: &Self::Element) -> Option<Self::Element> {
        if a.is_zero() {
            None // Zero has no inverse
        } else {
            Self::inverse(a, &self.n)
        }
    }

    fn pow(&self, a: &Self::Element, exp: &BigUint) -> Self::Element {
        Self::pow(a, exp, &self.n)
    }
}

impl fmt::Display for IntegerModN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Multiplicative Group of Integers Modulo {} (Z/{} Z)",
            self.n, self.n
        )
    }
}
