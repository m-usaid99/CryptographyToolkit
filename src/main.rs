mod finite_field;
mod polynomial;

use finite_field::FiniteField;
use polynomial::Polynomial;

// TODO:    - For Finite Field:
//              - See how to generate an irreducable poly, given a degree
//              - implement method to generate random poly (random finite field element)
//              - implement method to generate iterable list of polynomials (this is going to be tough)

fn main() {
    let start = std::time::Instant::now();

    // define a modulus
    let modulus = Polynomial::new(&[1, 0, 1, 1]);
    // Define a polynomial to invert (x^2 + x)
    let poly = Polynomial::new(&[0, 1, 1, 0]); // Represents x^2 + x

    if let Some(inv) = poly.inverse(&modulus) {
        println!("Inverse of {} mod {} is {}", poly, modulus, inv);

        // Verify that (poly * inv) mod modulus == 1
        let product = poly.multiply(&inv).modulo(&modulus);
        println!("Verification: (poly * inv) mod modulus = {}", product);

        assert_eq!(product, Polynomial::new(&[1])); // Should be 1
    } else {
        println!("No inverse exists for {} mod {}", poly, modulus);
    }
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
