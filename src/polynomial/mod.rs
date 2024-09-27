use bitvec::prelude::*;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

// Declare submodules
pub mod arithmetic;
pub mod euclidean;
pub mod irreducibility;
pub mod utils;

// Bring submodules into scope
//use arithmetic::*;
//use euclidean::*;
//use irreducibility::*;
//use utils::*;

/// Represents a polynomial over GF(2) with coefficients in Big-Endian (MSB first) order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub bits: BitVec<u8, Msb0>,
}

impl Polynomial {
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

// Implement Display trait
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let poly_str = self.to_string();
        write!(f, "{}", poly_str)
    }
}

// Implement operator overloading
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
        *self = &*self + other;
    }
}

impl<'a> MulAssign<&'a Polynomial> for Polynomial {
    fn mul_assign(&mut self, other: &'a Polynomial) {
        *self = &*self * other;
    }
}
