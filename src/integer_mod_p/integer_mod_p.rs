// src/integer_mod_p/integer_mod_p.rs

use crate::algebra::traits::{Algebra, Field, Group, Ring};
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use std::fmt;

/// Errors related to `IntegerModP`.
#[derive(Debug)]
pub enum IntegerModPError {
    NotPrime,
}

impl fmt::Display for IntegerModPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegerModPError::NotPrime => write!(f, "The modulus `p` must be a prime number."),
        }
    }
}

impl std::error::Error for IntegerModPError {}

/// Represents an integer modulo a prime number `p`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerModP {
    p: BigUint, // Prime modulus
}

impl IntegerModP {
    /// Creates a new `IntegerModP` with a given prime modulus `p`.
    pub fn new(p: BigUint) -> Result<Self, IntegerModPError> {
        if !Self::is_prime(&p) {
            return Err(IntegerModPError::NotPrime);
        }
        Ok(IntegerModP { p })
    }

    /// Simple primality check (inefficient for large p).
    fn is_prime(n: &BigUint) -> bool {
        if *n < 2.to_biguint().unwrap() {
            return false;
        }
        if *n == 2.to_biguint().unwrap() || *n == 3.to_biguint().unwrap() {
            return true;
        }
        if n % 2.to_biguint().unwrap() == BigUint::zero() {
            return false;
        }
        let sqrt_n = n.sqrt();
        let mut i = 3.to_biguint().unwrap();
        while &i <= &sqrt_n {
            if n % &i == BigUint::zero() {
                return false;
            }
            i += 2.to_biguint().unwrap();
        }
        true
    }

    /// Computes the multiplicative inverse using the Extended Euclidean Algorithm.
    fn inverse(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        let (gcd, x, _) = Self::extended_gcd(a, p);
        if gcd != BigUint::one() {
            return None; // Inverse does not exist
        }
        Some((x % p) + p)
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

    /// Raises `base` to the power `exp` modulo `p` using exponentiation by squaring.
    fn pow(base: &BigUint, exp: u128, p: &BigUint) -> BigUint {
        base.modpow(&exp.to_biguint().unwrap(), p)
    }
}

impl Algebra for IntegerModP {
    type Element = BigUint;
}

impl Ring for IntegerModP {
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        (a + b) % &self.p
    }

    fn mul(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        (a * b) % &self.p
    }

    fn zero(&self) -> Self::Element {
        BigUint::zero()
    }

    fn one(&self) -> Self::Element {
        BigUint::one()
    }
}

impl Group for IntegerModP {
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
            Self::inverse(a, &self.p)
        }
    }

    fn pow(&self, a: &Self::Element, exp: u128) -> Self::Element {
        Self::pow(a, exp, &self.p)
    }
}

impl Field for IntegerModP {}

impl fmt::Display for IntegerModP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Integers Modulo {} (Z_{})", self.p, self.p)
    }
}
