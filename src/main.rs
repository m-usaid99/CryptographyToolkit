mod algebra;
mod binary_extension_field;
mod generic_vector;
mod integer_mod_n;
mod integer_mod_p;
mod polynomial;

use algebra::traits::{Group, Ring};
use binary_extension_field::BinaryExtensionField;
use integer_mod_p::IntegerModP;
use num_bigint::ToBigUint;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();

    println!("\n================== Testing Binary Fields GF(2^n) ==================");
    // autogenerate a field based on degree
    let field = BinaryExtensionField::new_auto(17)?;
    println!("{}", field);

    // Create two field elements
    let a = field.random_element();
    let b = field.random_element();
    println!("\na: {}", a);
    println!("b: {}", b);

    // Perform addition using the Ring trait
    let sum = field.add(&a, &b);
    println!("\nSum: {}", sum); // Expected: x + 1

    // Perform multiplication using the Ring trait
    let product = field.mul(&a, &b);
    println!("\nProduct: {}", product); // Expected: x

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
    let a_cubed = field.pow(&a, 9);
    println!("a^9: {}", a_cubed); // Expected: x

    println!("\n================== Testing Prime Fields Z_p ==================");
    let p: u128 = 112399138331;
    let f_p = IntegerModP::new(p.to_biguint().unwrap())?;
    println!("Created the prime field: {}", f_p);

    println!("\n");
    println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
    Ok(())
}
