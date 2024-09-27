mod finite_field;
mod polynomial;

use finite_field::{FiniteField, FiniteFieldError};
use polynomial::Polynomial;

// TODO:    - For Finite Field:
//              - See how to generate an irreducable poly, given a degree
//              - implement method to generate random poly (random finite field element)
//              - implement method to generate iterable list of polynomials (this is going to be tough)

fn main() {
    let start = std::time::Instant::now();

    // define a modulus
    let modulus = [1, 0, 1, 1];

    let f = FiniteField::new(3, &modulus);
    match f {
        Ok(finite_field) => println!("{} created.", finite_field),
        Err(err) => println!("{:?}", err),
    }

    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
