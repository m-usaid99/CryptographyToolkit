mod finite_field;
mod polynomial;

use crate::finite_field::FiniteField;

// TODO: - Add add_in_place, and multiply_in_place for Polynomial
//       - For Finite Field:
//          - See how to generate an irreducable poly, given a degree
//          - implement method to generate random poly (random finite field element)
//          - implement method to generate iterable list of polynomials (this is going to be tough)

fn main() {
    let start = std::time::Instant::now();

    // Define the modulus polynomial for GF(2^3): x^3 + x + 1
    let modulus_coeffs = vec![1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let field = FiniteField::new(3, &modulus_coeffs).expect("Failed to create FiniteField");
    println!("{}", field);
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
