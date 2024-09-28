use crate::polynomial::Polynomial;
use core::fmt;
use rand::Rng;

// TODO:
//       - make a generator to create iterable for finite field elements
//       - try to optimize modular exponentiation
//       - create a list of known irreducible_polys so that there are ideal polys for common fields

#[derive(Debug)]
pub enum FiniteFieldError {
    NonIrreducibleModulus,
    InvalidModulusDegree,
    UnableToGenerateModulus,
}

impl fmt::Display for FiniteFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FiniteFieldError::NonIrreducibleModulus => {
                write!(f, "The provided modulus is not irreducible.")
            }
            FiniteFieldError::InvalidModulusDegree => write!(
                f,
                "The degree of the modulus does not match the field degree."
            ),
            FiniteFieldError::UnableToGenerateModulus => {
                write!(f, "Unable to generate an irreducible modulus polynomial.")
            }
        }
    }
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
    pub fn new_auto(n: usize) -> Result<FiniteField, FiniteFieldError> {
        let max_attempts = 3000; // Adjust as needed
        match Polynomial::irreducible_element(n, max_attempts) {
            Some(modulus) => Ok(FiniteField { n, modulus }),
            None => Err(FiniteFieldError::UnableToGenerateModulus),
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
    pub fn mod_exp(&self, a: &Polynomial, exp: u64) -> Polynomial {
        let one = Polynomial::one();
        let mut result = one.clone();
        let mut base = a.clone();
        let mut exponent = exp;

        while exponent > 0 {
            if exponent & 1 == 1 {
                result = self.multiply(&result, &base);
            }
            // Use optimized squaring
            base = base.square();
            base = self.modulo(&base);
            exponent >>= 1;
        }
        result
    }
}

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Galois field of size 2^{} with modulus {}",
            self.n, self.modulus
        )
    }
}
