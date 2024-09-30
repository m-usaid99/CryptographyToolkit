// src/finite_field/iterators.rs

use crate::polynomial::Polynomial;

pub struct BinaryExtensionFieldElementDegreeBoundedIterator {
    pub current: u128,
    pub max: u128,
    pub degree: usize,
    pub max_degree: usize,
}

impl Iterator for BinaryExtensionFieldElementDegreeBoundedIterator {
    type Item = Polynomial;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.max {
            let bits = (0..self.degree)
                .rev()
                .map(|i| ((self.current >> i) & 1) as u8)
                .collect::<Vec<_>>();

            let poly = Polynomial::new(&bits);
            self.current += 1;

            if poly.degree() <= self.max_degree {
                return Some(poly);
            }
        }
        None
    }
}

pub struct BinaryExtensionFieldElementFixedDegreeIterator {
    pub current: u128,
    pub max: u128,
    pub degree: usize,
    pub target_degree: usize,
}

impl Iterator for BinaryExtensionFieldElementFixedDegreeIterator {
    type Item = Polynomial;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.max {
            let bits = (0..self.degree)
                .rev()
                .map(|i| ((self.current >> i) & 1) as u8)
                .collect::<Vec<_>>();

            let poly = Polynomial::new(&bits);
            self.current += 1;

            if poly.degree() == self.target_degree {
                return Some(poly);
            }
        }
        None
    }
}
