use super::Polynomial;
use bitvec::prelude::*;

impl Polynomial {
    /// Checks if the polynomial is irreducible over GF(2) using Rabin's Test
    pub fn is_irreducible(&self) -> bool {
        let n = self.degree();
        if n <= 0 {
            return false; // Degree must be at least 1
        }
        if n == 1 {
            return true; // All degree 1 polynomials are irreducible
        }

        let prime_factors = Polynomial::distinct_prime_factors(n);
        let x = Polynomial::x();

        for &p in &prime_factors {
            let exponent = n / p;
            let x_exp = x.pow2_mod(exponent, self);
            let diff = x_exp.add(&x); // x^{2^{n/p}} - x == x^{2^{n/p}} + x in GF(2)
            let gcd = self.gcd(&diff);
            if gcd.degree() >= 1 {
                return false; // Reducible
            }
        }

        // Finally, check that x^{2^n} mod f(x) == x
        let x_final = x.pow2_mod(n, self);
        let condition = x_final.add(&x) == Polynomial::new(&[0]); // Should equal 0
        condition
    }

    // generate all possible trinomials
    pub fn generate_trinomials(degree: usize) -> Vec<Polynomial> {
        let mut trinomials = Vec::new();
        for k in (1..degree).rev() {
            let mut coeffs = vec![0u8; degree + 1];
            coeffs[degree] = 1;
            coeffs[k] = 1;
            coeffs[0] = 1;
            trinomials.push(Polynomial::new(&coeffs));
        }
        trinomials
    }

    /// returns an irreducible trinomial if exists
    pub fn irreducible_trinomial(degree: usize) -> Option<Polynomial> {
        let tris = Polynomial::generate_trinomials(degree);
        println!("{:?}", tris);
        for trinomial in tris {
            if trinomial.is_irreducible() {
                return Some(trinomial);
            }
        }
        None
    }

    /// generate an irreducible monic of degree `n` to serve as modulus for finite field
    pub fn irreducible_element(degree: usize) -> Option<Polynomial> {
        // try to find irreducible trinomial
        if let Some(tri_poly) = Polynomial::irreducible_trinomial(degree) {
            return Some(tri_poly);
        }

        // if no irreducible trinomial, enumerate all possible monics
        for poly in Polynomial::generate_all_monics(degree) {
            if poly.is_irreducible() {
                return Some(poly);
            }
        }
        None
    }
}
