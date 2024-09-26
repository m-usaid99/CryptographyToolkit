mod finite_field;
mod polynomial;

use crate::finite_field::FiniteField;

fn main() {
    let start = std::time::Instant::now();

    // Define the modulus polynomial for GF(2^3): x^3 + x + 1
    let modulus_coeffs = vec![1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let field = FiniteField::new(3, &modulus_coeffs).expect("Failed to create FiniteField");
    println!("{}", field);
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
