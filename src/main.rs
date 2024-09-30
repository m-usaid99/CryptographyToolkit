mod algebra;
mod finite_field;
mod polynomial;

use algebra::traits::{Field, Group, Ring};
use finite_field::FiniteField;
use finite_field::FiniteFieldError;
use polynomial::Polynomial;

fn main() -> Result<(), FiniteFieldError> {
    let start = std::time::Instant::now();
    // Define a finite field GF(2^3) with modulus x^3 + x + 1
    let field = FiniteField::new(3, &[1, 0, 1, 1])?; // Coefficients in big-endian: x^3 + x + 1
    println!("{}", field);

    // Create two field elements
    let a = Polynomial::new(&[1, 0, 1]); // Represents x^2 + 1
    let b = Polynomial::new(&[1, 1, 0]); // Represents x^2 + x

    // Perform addition using the Ring trait
    let sum = field.add(&a, &b);
    println!("Sum: {}", sum); // Expected: x + 1

    // Perform multiplication using the Ring trait
    let product = field.mul(&a, &b);

    println!("Product: {}", product); // Expected: x

    // Find inverse using the Group trait
    if let Some(inv_a) = field.inverse(&a) {
        println!("Inverse of a: {}", inv_a);
        // Verify that a * inv_a = 1
        let verification = field.combine(&a, &inv_a);
        println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    } else {
        println!("a has no inverse in the field.");
    }

    // Perform exponentiation using the Field trait
    let a_cubed = field.pow(&a, 3);
    println!("a^3: {}", a_cubed); // Expected: x

    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
    Ok(())
}
