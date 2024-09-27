// src/polynomial/arithmetic.rs

use super::Polynomial;
use bitvec::prelude::*;

impl Polynomial {
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = usize::max(self.bits.len(), other.bits.len());
        let mut result_bits = BitVec::<u8, Msb0>::with_capacity(max_len);

        let self_offset = max_len - self.bits.len();
        let other_offset = max_len - other.bits.len();

        for i in 0..max_len {
            let self_bit = if i >= self_offset {
                self.bits[i - self_offset]
            } else {
                false
            };

            let other_bit = if i >= other_offset {
                other.bits[i - other_offset]
            } else {
                false
            };

            result_bits.push(self_bit ^ other_bit);
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        Polynomial { bits: result_bits }
    }

    /// Adds another polynomial to `self` in place.
    pub fn add_in_place(&mut self, other: &Polynomial) {
        let max_len = usize::max(self.bits.len(), other.bits.len());

        let self_len = self.bits.len();
        let other_len = other.bits.len();

        // Resize `self.bits` to the maximum length
        self.bits.resize(max_len, false);

        // Create a new BitVec to hold the result
        let mut result_bits = BitVec::<u8, Msb0>::with_capacity(max_len);

        // Iterate over the bits from most significant to least significant
        for i in 0..max_len {
            let self_idx = if i >= max_len - self_len {
                Some(i - (max_len - self_len))
            } else {
                None
            };

            let other_idx = if i >= max_len - other_len {
                Some(i - (max_len - other_len))
            } else {
                None
            };

            let self_bit = self_idx
                .and_then(|idx| self.bits.get(idx))
                .map(|b| *b)
                .unwrap_or(false);
            let other_bit = other_idx
                .and_then(|idx| other.bits.get(idx))
                .map(|b| *b)
                .unwrap_or(false);

            result_bits.push(self_bit ^ other_bit);
        }

        // Update self.bits with the result
        self.bits = result_bits;

        // Trim leading zeros
        while self.bits.len() > 1 && !self.bits.first().unwrap() {
            self.bits.remove(0);
        }
    }

    // multiplies itself with another polynomial and returns the resultant
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        let result_len = self.bits.len() + other.bits.len() - 1;
        let mut result_bits = BitVec::<u8, Msb0>::repeat(false, result_len);

        for i in 0..self.bits.len() {
            if self.bits[i] {
                for j in 0..other.bits.len() {
                    if other.bits[j] {
                        let idx = i + j;
                        let bit = result_bits[idx];
                        result_bits.set(idx, bit ^ true);
                    }
                }
            }
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        Polynomial { bits: result_bits }
    }

    /// Multiplies `self` with another polynomial in place.
    pub fn multiply_in_place(&mut self, other: &Polynomial) {
        let self_len = self.bits.len();
        let other_len = other.bits.len();
        let result_len = self_len + other_len - 1;
        let mut result_bits = bitvec![u8, Msb0; 0; result_len];

        for i in 0..self_len {
            if self.bits[i] {
                for j in 0..other_len {
                    if other.bits[j] {
                        let idx = i + j;

                        // Perform XOR
                        let result_bit = result_bits[idx];
                        result_bits.set(idx, result_bit ^ true);
                    }
                }
            }
        }

        // Trim leading zeros
        while result_bits.len() > 1 && !result_bits.first().unwrap() {
            result_bits.remove(0);
        }

        // Update self.bits
        self.bits = result_bits;
    }

    pub fn square(&self) -> Polynomial {
        let mut squared_bits = BitVec::<u8, Msb0>::with_capacity(self.bits.len() * 2);

        for bit in self.bits.iter() {
            squared_bits.push(*bit);
            squared_bits.push(false); // Insert a zero after each bit
        }

        // Remove trailing zero if added extra
        if squared_bits.len() > 0 && !squared_bits.last().unwrap() {
            squared_bits.pop();
        }

        // Trim leading zeros
        while squared_bits.len() > 1 && !squared_bits.first().unwrap() {
            squared_bits.remove(0);
        }

        Polynomial { bits: squared_bits }
    }

    pub fn modulo(&self, modulus: &Polynomial) -> Polynomial {
        let mut remainder = self.clone();

        while remainder.degree() >= modulus.degree() && !remainder.is_zero() {
            let degree_diff = remainder.degree() - modulus.degree();
            let shifted_modulus = modulus.shift_left(degree_diff);

            // Subtract (XOR in GF(2))
            remainder = remainder.add(&shifted_modulus);

            // Trim leading zeros
            while remainder.bits.len() > 1 && !remainder.bits.first().unwrap() {
                remainder.bits.remove(0);
            }
        }

        remainder
    }
}
