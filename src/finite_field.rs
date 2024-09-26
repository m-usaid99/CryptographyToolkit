use crate::polynomial::Polynomial;

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

impl FiniteField {}
