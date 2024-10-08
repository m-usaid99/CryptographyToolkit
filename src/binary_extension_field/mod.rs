// src/finite_field/mod.rs

use crate::algebra::traits::{Algebra, Field, Group, Ring};
pub use crate::binary_extension_field::errors::BinaryExtensionFieldError;
use crate::polynomial::Polynomial;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::Rng;
use std::fmt;

mod errors;
mod iterators;

pub use iterators::*;

// TODO:    - create a list of known irreducible_polys so that there are ideal polys for common fields
//          - write unit tests for each function

#[derive(Debug, Clone)]
pub struct BinaryExtensionField {
    n: usize,
    modulus: Polynomial,
}

impl BinaryExtensionField {
    /// Creates a new FiniteField with degree `n` and the given modulus modulus polynomial
    /// coefficients
    ///
    /// # Arguments
    ///
    /// * `n` - The degree of the extension field
    /// * `modulus_coeffs` - A slice of 0s and 1s that represents the coefficients of the modulus
    /// polynomial in big-endian order
    ///
    /// # Returns
    ///
    /// * `Ok(FiniteField)` if the modulus is appropriate
    /// * `Err(FiniteFieldError)` if the modulus is not irreducible or degree mismatch.
    pub fn new(n: usize, modulus_coeffs: &[u8]) -> Result<Self, BinaryExtensionFieldError> {
        let modulus = Polynomial::new(modulus_coeffs);
        if modulus.degree() != n {
            return Err(BinaryExtensionFieldError::InvalidModulusDegree);
        }
        if !modulus.is_irreducible() {
            return Err(BinaryExtensionFieldError::NonIrreducibleModulus);
        }
        Ok(BinaryExtensionField { n, modulus })
    }

    /// Creates a new FiniteField with degree `n` by automatically generating an irreducible modulus polynomial.
    ///
    /// # Arguments
    ///
    /// * `n` - The degree of the extension field.
    ///
    /// # Returns
    ///
    /// * `Ok(FiniteField)` if an irreducible modulus is successfully generated.
    /// * `Err(FiniteFieldError)` if no irreducible modulus is found within the maximum attempts.
    pub fn new_auto(n: usize) -> Result<BinaryExtensionField, BinaryExtensionFieldError> {
        let max_attempts = 3000; // Adjust as needed
        match Polynomial::irreducible_element(n, max_attempts) {
            Some(modulus) => Ok(BinaryExtensionField { n, modulus }),
            None => Err(BinaryExtensionFieldError::UnableToGenerateModulus),
        }
    }

    /// Returns a reference to the modulus polynomial.
    pub fn modulus(&self) -> &Polynomial {
        &self.modulus
    }

    /// Adds two field elements together
    ///
    /// # Arguments
    ///
    /// * `a` - First field element
    /// * `b` - Second field element
    ///
    /// # Returns
    ///
    /// The sum of `a` and `b` in the field.
    pub fn add(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        a.add(b)
    }

    /// Multiplies two field elements together with modulo reduction
    ///
    /// # Arguments
    ///
    /// * `a` - First field element
    /// * `b` - Second field element
    ///
    /// # Returns
    ///
    /// The product of `a` and `b` in the field.
    pub fn multiply(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        let product = a.multiply(b);
        self.modulo(&product)
    }

    /// Finds the multiplicative inverse of a field element.
    ///
    /// # Arguments
    ///
    /// * `a` - The field element to invert.
    ///
    /// # Returns
    ///
    /// `Some(inverse)` if the inverse exists, otherwise `None`.
    pub fn inverse(&self, a: &Polynomial) -> Option<Polynomial> {
        a.inverse(&self.modulus)
    }

    /// Performs modulo reduction with the field's modulus polynomial.
    fn modulo(&self, poly: &Polynomial) -> Polynomial {
        poly.modulo(&self.modulus)
    }

    /// Generates a random element in the finite field.
    ///
    /// # Returns
    ///
    /// A random polynomial representing an element of the finite field.
    pub fn random_element(&self) -> Polynomial {
        let mut rng = rand::thread_rng();
        let degree = self.n;
        let mut coeffs = vec![];
        for _ in 0..degree {
            coeffs.push(rng.gen_range(0..=1));
        }
        Polynomial::new(&coeffs)
    }

    /// Raises a field element to the power `exp` using exponentiation by squaring.
    ///
    /// # Arguments
    ///
    /// * `a` - The base field element.
    /// * `exp` - The exponent (non-negative integer).
    ///
    /// # Returns
    ///
    /// The result of \( a^{\text{exp}} \) in the field.
    pub fn mod_exp(&self, a: &Polynomial, exp: &BigUint) -> Polynomial {
        let one = Polynomial::one();
        let mut result = one.clone();
        let mut base = a.clone();
        let mut exponent = exp.clone();

        while !exponent.is_zero() {
            if &exponent & BigUint::one() == BigUint::one() {
                result = self.multiply(&result, &base);
            }
            // Use optimized squaring
            base = base.square();
            base = self.modulo(&base);
            exponent >>= 1;
        }
        result
    }

    /// Returns an iterator over all elements of the finite field up to a given degree `max_degree`.
    ///
    /// # Arguments
    ///
    /// * `max_degree` - The maximum degree of the polynomials to generate (less than or equal to field degree).
    ///
    /// # Returns
    ///
    /// An iterator over polynomials of degrees up to `max_degree`.
    pub fn elements_up_to_degree(
        &self,
        max_degree: usize,
    ) -> BinaryExtensionFieldElementDegreeBoundedIterator {
        if max_degree > self.n {
            panic!("max_degree cannot exceed the field degree");
        }
        BinaryExtensionFieldElementDegreeBoundedIterator {
            current: 0,
            max: 1 << self.n,
            degree: self.n,
            max_degree,
        }
    }

    /// Returns an iterator over all elements of the finite field of a specific degree `target_degree`.
    pub fn elements_of_degree(
        &self,
        target_degree: usize,
    ) -> BinaryExtensionFieldElementFixedDegreeIterator {
        if target_degree > self.n {
            panic!("target_degree cannot exceed the field degree");
        }
        BinaryExtensionFieldElementFixedDegreeIterator {
            current: 0,
            max: 1 << self.n,
            degree: self.n,
            target_degree,
        }
    }
}

impl fmt::Display for BinaryExtensionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Galois field of size 2^{} with modulus {}",
            self.n, self.modulus
        )
    }
}

impl Algebra for BinaryExtensionField {
    type Element = Polynomial;
}

impl Ring for BinaryExtensionField {
    fn add(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        self.add(a, b)
    }

    fn mul(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        self.multiply(a, b)
    }

    fn zero(&self) -> Self::Element {
        Polynomial::zero(self.n)
    }

    fn one(&self) -> Self::Element {
        Polynomial::one()
    }
}

impl Group for BinaryExtensionField {
    fn combine(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        self.multiply(a, b)
    }

    fn identity(&self) -> Self::Element {
        Polynomial::one()
    }

    fn inverse(&self, a: &Self::Element) -> Option<Self::Element> {
        self.inverse(a)
    }

    fn pow(&self, a: &Self::Element, exp: &BigUint) -> Self::Element {
        self.mod_exp(a, exp)
    }
}

impl Field for BinaryExtensionField {}
