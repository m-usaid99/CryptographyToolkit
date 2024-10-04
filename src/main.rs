mod algebra;
mod binary_extension_field;
mod generic_vector;
mod integer_mod_n;
mod integer_mod_p;
mod polynomial;

use algebra::traits::{Group, Ring};
use binary_extension_field::BinaryExtensionField;
use integer_mod_n::IntegerModN;
use integer_mod_p::IntegerModP;
use num_bigint::ToBigUint;
use std::error::Error;

// TODO: - implement an "in" method to check if element belongs in struct

fn main() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();

    println!("\n================== Testing Binary Fields GF(2^n) ==================");
    // autogenerate a field based on degree
    let field = BinaryExtensionField::new_auto(8)?;
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

    let el0 = f_p.random_element();
    let el1 = f_p.random_element();

    println!("First random element a: {}", el0);
    println!("Second random element b: {}", el1);

    let sum = f_p.add(&el0, &el1);
    println!("Sum: {}", sum);

    let product = f_p.mul(&el0, &el1);
    println!("Product: {}", product);

    let exp: u128 = 655997;
    let a_exp = f_p.pow(&el0, exp);
    println!("a^{}: {}", exp, a_exp);

    if let Some(inv_a) = f_p.inverse(&el0) {
        println!("Inverse of a: {}", inv_a);
        let verification = f_p.combine(&el0, &inv_a);
        println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    } else {
        println!("a has no inverse in this prime field.");
    }

    println!("\n=========== Testing Group of Integers Mod n (Z/nZ) ============");

    let group = IntegerModN::new((p - 1).to_biguint().unwrap());
    println!("{}", group);

    let a = group.random_group_element();
    let b = group.random_group_element();

    println!("a: {} belonging to {}", a, group);
    println!("b: {} belonging to {}", b, group);

    let sum = group.add(&a, &b);
    println!("Sum: {}", sum);

    let product = group.mul(&a, &b);
    println!("Product: {}", product);

    let exp: u128 = 1123019;
    let a_exp = group.pow(&a, exp);
    println!("a^{}: {}", exp, a_exp);

    // Find inverse using the Group trait
    if let Some(inv_a) = group.inverse(&a) {
        println!("Inverse of a: {}", inv_a);
        // Verify that a * inv_a = 1
        let verification = group.combine(&a, &inv_a);
        println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    } else {
        println!("a has no inverse in the field.");
    }

    println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
    Ok(())
}
