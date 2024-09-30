// src/finite_field/errors.rs

use std::fmt;

#[derive(Debug)]
pub enum BinaryExtensionFieldError {
    NonIrreducibleModulus,
    InvalidModulusDegree,
    UnableToGenerateModulus,
}

impl fmt::Display for BinaryExtensionFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryExtensionFieldError::NonIrreducibleModulus => {
                write!(f, "The provided modulus is not irreducible.")
            }
            BinaryExtensionFieldError::InvalidModulusDegree => write!(
                f,
                "The degree of the modulus does not match the field degree."
            ),
            BinaryExtensionFieldError::UnableToGenerateModulus => {
                write!(f, "Unable to generate an irreducible modulus polynomial.")
            }
        }
    }
}

impl std::error::Error for BinaryExtensionFieldError {}
