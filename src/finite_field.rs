use crate::polynomial::Polynomial;
use core::fmt;
use rand::Rng;

#[derive(Debug)]
pub enum FiniteFieldError {
    NonIrreducibleModulus,
    InvalidModulusDegree,
}

#[derive(Debug, Clone)]
pub struct FiniteField {
    n: usize,
    modulus: Polynomial,
}

impl FiniteField {
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
    pub fn new(n: usize, modulus_coeffs: &[u8]) -> Result<Self, FiniteFieldError> {
        let modulus = Polynomial::new(modulus_coeffs);
        if modulus.degree() != n {
            return Err(FiniteFieldError::InvalidModulusDegree);
        }
        if !modulus.is_irreducible() {
            return Err(FiniteFieldError::NonIrreducibleModulus);
        }
        Ok(FiniteField { n, modulus })
    }

    //pub fn irreducible_element(degree: usize) -> Option<Polynomial> {
    //    let mut rng = rand::thread_rng();
    //    if degree < 1 {
    //        return None;
    //    }
    //
    //    loop {
    //        let mut coeffs = vec![0u8; degree + 1];
    //        coeffs[0] = 1;
    //    }
    //}

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
    pub fn add(&self, a: Polynomial, b: &Polynomial) -> Polynomial {
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
    pub fn inverse(&self, a: Polynomial) -> Option<Polynomial> {
        a.inverse(&self.modulus)
    }

    /// Performs modulo reduction with the field's modulus polynomial.
    fn modulo(&self, poly: &Polynomial) -> Polynomial {
        poly.modulo(&self.modulus)
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
    pub fn exponentiate(&self, a: &Polynomial, exp: u64) -> Polynomial {
        let one = Polynomial::new(&[1]);
        let mut result = one.clone();
        let mut base = a.clone();
        let mut exponent = exp;

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = self.multiply(&result, &base);
            }
            base = self.multiply(&base, &base);
            exponent >>= 1;
        }
        result
    }
}

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GF(2^{}) with modulus {}", self.n, self.modulus)
    }
}
