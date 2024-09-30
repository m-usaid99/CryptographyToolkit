// src/integer_mod_n/integer_mod_n.rs

use crate::algebra::traits::{Algebra, Group, Ring};
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
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
        Some((x % n) + n)
    }

    /// Extended Euclidean Algorithm.
    /// Returns a tuple of (gcd, x, y) such that ax + by = gcd(a, b)
    fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
        let mut old_r = a.clone();
        let mut r = b.clone();
        let mut old_s = BigUint::one();
        let mut s = BigUint::zero();
        let mut old_t = BigUint::zero();
        let mut t = BigUint::one();

        while r != BigUint::zero() {
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

        (old_r, old_s, old_t)
    }

    /// Raises `base` to the power `exp` modulo `n` using exponentiation by squaring.
    pub fn pow(base: &BigUint, exp: u128, n: &BigUint) -> BigUint {
        base.modpow(&exp.to_biguint().unwrap(), n)
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

    fn pow(&self, a: &Self::Element, exp: u128) -> Self::Element {
        Self::pow(a, exp, &self.n)
    }
}

impl fmt::Display for IntegerModN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Integer Modulo {} (Z/{}Z)", self.n, self.n)
    }
}
