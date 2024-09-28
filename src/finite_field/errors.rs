use std::fmt;

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

impl std::error::Error for FiniteFieldError {}
