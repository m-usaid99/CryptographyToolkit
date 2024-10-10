// src/integer_mod_p/integer_mod_p.rs

use crate::algebra::traits::{Algebra, Field, Group, Ring};
use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::OsRng;
use rand::{self, Rng};
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
    pub p: BigUint, // Prime modulus
}

impl IntegerModP {
    /// Creates a new `IntegerModP` with a given prime modulus `p`.
    pub fn new(p: BigUint) -> Result<Self, IntegerModPError> {
        if !Self::is_prime_miller_rabin(&p, 50) {
            return Err(IntegerModPError::NotPrime);
        }
        Ok(IntegerModP { p })
    }

    pub fn new_valid_prime(p: BigUint) -> Self {
        IntegerModP { p }
    }

    /// Generates a random element in the field Z/pZ.
    pub fn random_element(&self) -> BigUint {
        let mut rng = OsRng;
        let bytes = self.p.to_bytes_be();
        let mut random_bytes = vec![0u8; bytes.len()];
        rng.fill(&mut random_bytes[..]);
        let random_num = BigUint::from_bytes_be(&random_bytes);
        &random_num % &self.p
    }

    /// Miller-Rabin primality test.
    /// Returns `true` if `n` is probably prime.
    /// `k` is the number of testing rounds.
    fn is_prime_miller_rabin(n: &BigUint, k: u32) -> bool {
        // Handle base cases
        if *n < BigUint::from(2u32) {
            return false;
        }
        if *n == BigUint::from(2u32) || *n == BigUint::from(3u32) {
            return true;
        }
        if n % 2u32 == BigUint::zero() {
            return false;
        }

        // Write n - 1 as 2^s * d
        let mut d = n - 1u32;
        let mut s = 0u32;
        while &d % 2u32 == BigUint::zero() {
            d /= 2u32;
            s += 1;
        }

        let mut rng = OsRng;

        'witness_loop: for _ in 0..k {
            // Choose a random base a in [2, n - 2]
            let a = rng.gen_biguint_range(&BigUint::from(2u32), &(n - 2u32));
            let mut x = a.modpow(&d, n);

            if x == BigUint::one() || x == (n - 1u32) {
                continue;
            }

            for _ in 0..(s - 1) {
                x = x.modpow(&BigUint::from(2u32), n);

                if x == (n - 1u32) {
                    continue 'witness_loop;
                }
            }

            // Composite
            return false;
        }

        // Probably prime
        true
    }

    /// Computes the multiplicative inverse using the Extended Euclidean Algorithm.
    pub fn inverse(a: &BigUint, p: &BigUint) -> Option<BigUint> {
        let (gcd, x, _) = Self::extended_gcd(a, p);
        if gcd != BigUint::one() {
            return None; // Inverse does not exist
        }

        // Compute x mod p to ensure the inverse is positive
        let p_bigint = p.to_bigint().unwrap();
        let x_mod_p = x.mod_floor(&p_bigint).to_biguint().unwrap();
        Some(x_mod_p)
    }

    /// Subtracts `b` from `a` modulo `p`.
    pub fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        if a >= b {
            (a - b) % &self.p
        } else {
            (&self.p + a - b) % &self.p
        }
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

    /// Raises `base` to the power `exp` modulo `p` using exponentiation by squaring.
    pub fn pow(base: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
        base.modpow(&exp.to_biguint().unwrap(), p)
    }

    /// Checks if a given number is an element of Z/pZ.
    /// Accepts any type that can be converted into a `BigUint`.
    pub fn contains<T: Into<BigUint>>(&self, num: T) -> bool {
        let num = num.into();
        num < self.p
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

    fn pow(&self, a: &Self::Element, exp: &BigUint) -> Self::Element {
        a.modpow(exp, &self.p)
    }
}

impl Field for IntegerModP {}

impl fmt::Display for IntegerModP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Prime field of Integers Modulo {} (Z_{})",
            self.p, self.p
        )
    }
}
