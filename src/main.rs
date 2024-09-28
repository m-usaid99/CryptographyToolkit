mod finite_field;
mod polynomial;

use finite_field::FiniteField;

fn main() {
    let start = std::time::Instant::now();

    let field = FiniteField::new_auto(8).expect("Failed to create finite field");

    println!("{} created.", field);

    let a = field.random_element();
    let b = field.random_element();

    println!("a = {}", a);
    println!("b = {}", b);

    let sum = field.add(&a, &b);
    println!("a + b = {}", sum);

    let product = field.multiply(&a, &b);
    println!("a * b = {}", product);

    if let Some(inv_a) = field.inverse(&a) {
        println!("Inverse of a = {}", inv_a);
        let check = field.multiply(&a, &inv_a);
        println!("a * a^(-1) = {}", check); // Should be 1
    } else {
        println!("a has no multiplicative inverse.");
    }
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
