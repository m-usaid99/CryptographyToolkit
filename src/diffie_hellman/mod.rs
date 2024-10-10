// src/diffie_hellman/diffie_hellman.rs

use crate::algebra::traits::Group;
use crate::integer_mod_p::IntegerModP;
use num_bigint::BigUint;
use num_traits::One;
use rand::rngs::OsRng;
use rand::RngCore;
use std::error::Error;

pub struct DiffieHellman {
    pub field: IntegerModP, // Finite field ℤₚ
    pub g: BigUint,         // Generator
}

impl DiffieHellman {
    pub fn new(p: BigUint, g: BigUint) -> Result<Self, Box<dyn Error>> {
        let field = IntegerModP::new_valid_prime(p);
        Ok(DiffieHellman { field, g })
    }

    pub fn generate_private_key(&self) -> BigUint {
        let mut rng = OsRng;
        loop {
            let mut bytes = vec![0u8; self.field.p.bits() as usize / 8 + 1];
            rng.fill_bytes(&mut bytes);
            let private_key = BigUint::from_bytes_be(&bytes) % &self.field.p;
            if private_key > BigUint::one() && private_key < &self.field.p - BigUint::one() {
                return private_key;
            }
        }
    }

    pub fn compute_public_key(&self, private_key: &BigUint) -> BigUint {
        self.field.pow(&self.g, private_key)
    }

    /// Computes the shared secret: (peer_public)^private mod p
    pub fn compute_shared_secret(&self, peer_public: &BigUint, private_key: &BigUint) -> BigUint {
        self.field.pow(peer_public, private_key)
    }
}
