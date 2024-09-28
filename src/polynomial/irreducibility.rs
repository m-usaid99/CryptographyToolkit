use super::Polynomial;
use rand::Rng;

impl Polynomial {
    /// Checks if the polynomial is irreducible over GF(2) using Rabin's Test.
    pub fn is_irreducible(&self) -> bool {
        let n = self.degree();
        if n <= 0 {
            return false;
        }
        if n == 1 {
            return true;
        }

        let x = Polynomial::x();
        let mut x_power = x.clone();

        for _ in 0..(n / 2) {
            x_power = x_power.pow2_mod(1, self);
            let gcd = x_power.add(&x).gcd(self);
            if !gcd.is_one() {
                return false;
            }
        }
        true
    }

    pub fn irreducible_element(degree: usize, max_attempts: usize) -> Option<Polynomial> {
        for _ in 0..max_attempts {
            let poly = Polynomial::random_monic_polynomial(degree);
            if poly.is_irreducible() {
                return Some(poly);
            }
        }
        None
    }

    /// Generates a random monic polynomial of a specified degree.
    pub fn random_monic_polynomial(degree: usize) -> Polynomial {
        let mut rng = rand::thread_rng();
        let mut coeffs = vec![1u8]; // Leading coefficient is 1
        for _ in 0..degree {
            coeffs.push(rng.gen_range(0..=1));
        }
        Polynomial::new(&coeffs)
    }
}
