mod algebra;
mod binary_extension_field;
mod diffie_hellman;
mod generic_vector;
mod integer_mod_n;
mod integer_mod_p;
mod polynomial;

use algebra::traits::{Group, Ring};
use binary_extension_field::BinaryExtensionField;
use diffie_hellman::DiffieHellman;
use integer_mod_n::IntegerModN;
use integer_mod_p::IntegerModP;

use hex::decode;
use num_bigint::{BigUint, ToBigUint};
use std::error::Error;

// TODO: - implement an "in" method to check if element belongs in struct

fn main() -> Result<(), Box<dyn Error>> {
    let start = std::time::Instant::now();

    //println!("\n================== Testing Binary Fields GF(2^n) ==================");
    //// autogenerate a field based on degree
    //let field = BinaryExtensionField::new_auto(8)?;
    //println!("{}", field);
    //
    //// Create two field elements
    //let a = field.random_element();
    //let b = field.random_element();
    //println!("\na: {}", a);
    //println!("b: {}", b);
    //
    //// Perform addition using the Ring trait
    //let sum = field.add(&a, &b);
    //println!("\nSum: {}", sum); // Expected: x + 1
    //
    //// Perform multiplication using the Ring trait
    //let product = field.mul(&a, &b);
    //println!("\nProduct: {}", product); // Expected: x
    //
    //// Find inverse using the Group trait
    //if let Some(inv_a) = field.inverse(&a) {
    //    println!("Inverse of a: {}", inv_a);
    //    // Verify that a * inv_a = 1
    //    let verification = field.combine(&a, &inv_a);
    //    println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    //} else {
    //    println!("a has no inverse in the field.");
    //}
    //
    //// Perform exponentiation using the Field trait
    //let a_cubed = field.pow(&a, &9.to_biguint().unwrap());
    //println!("a^9: {}", a_cubed); // Expected: x
    //
    //println!("\n================== Testing Prime Fields Z_p ==================");
    //let p: u128 = 112399138331;
    //let f_p = IntegerModP::new(p.to_biguint().unwrap())?;
    //println!("Created the prime field: {}", f_p);
    //
    //let el0 = f_p.random_element();
    //let el1 = f_p.random_element();
    //
    //println!("First random element a: {}", el0);
    //println!("Second random element b: {}", el1);
    //
    //let sum = f_p.add(&el0, &el1);
    //println!("Sum: {}", sum);
    //
    //let product = f_p.mul(&el0, &el1);
    //println!("Product: {}", product);
    //
    //let exp: u128 = 655997;
    //let a_exp = f_p.pow(&el0, &exp.to_biguint().unwrap());
    //println!("a^{}: {}", exp, a_exp);
    //
    //if let Some(inv_a) = f_p.inverse(&el0) {
    //    println!("Inverse of a: {}", inv_a);
    //    let verification = f_p.combine(&el0, &inv_a);
    //    println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    //} else {
    //    println!("a has no inverse in this prime field.");
    //}
    //
    //println!("\n=========== Testing Group of Integers Mod n (Z/nZ) ============");
    //
    //let group = IntegerModN::new((p - 1).to_biguint().unwrap());
    //println!("{}", group);
    //
    //let a = group.random_group_element();
    //let b = group.random_group_element();
    //
    //println!("a: {} belonging to {}", a, group);
    //println!("b: {} belonging to {}", b, group);
    //
    //let sum = group.add(&a, &b);
    //println!("Sum: {}", sum);
    //
    //let product = group.mul(&a, &b);
    //println!("Product: {}", product);
    //
    //let exp: u128 = 1123019;
    //let a_exp = group.pow(&a, &exp.to_biguint().unwrap());
    //println!("a^{}: {}", exp, a_exp);
    //
    //// Find inverse using the Group trait
    //if let Some(inv_a) = group.inverse(&a) {
    //    println!("Inverse of a: {}", inv_a);
    //    // Verify that a * inv_a = 1
    //    let verification = group.combine(&a, &inv_a);
    //    println!("a * inv_a: {}", verification); // Should print the multiplicative identity
    //} else {
    //    println!("a has no inverse in the field.");
    //}
    //
    println!("\n=========== Testing Diffie-Hellmann ============");

    let p_hex = concat!(
        "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1",
        "29024E088A67CC74020BBEA63B139B22514A08798E3404DD",
        "EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245",
        "E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED",
        "EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE65381",
        "FFFFFFFFFFFFFFFF"
    );
    // Decode hex to bytes
    let p_bytes = decode(p_hex)?;
    // Convert bytes to BigUint
    let p = BigUint::from_bytes_be(&p_bytes);
    let g = BigUint::from(2u32); // Generator
                                 //
    let dh = diffie_hellman::DiffieHellman::new(p, g)?;
    let alice_private = dh.generate_private_key();
    let alice_public = dh.compute_public_key(&alice_private);
    let bob_private = dh.generate_private_key();
    let bob_public = dh.compute_public_key(&bob_private);
    let alice_shared = dh.compute_shared_secret(&bob_public, &alice_private);
    let bob_shared = dh.compute_shared_secret(&alice_public, &bob_private);

    println!("Alice's Private Key: {}", alice_private);
    println!("Alice's Public Key: {}", alice_public);
    println!("Bob's Private Key: {}", bob_private);
    println!("Bob's Public Key: {}", bob_public);
    println!("Alice's Shared Secret: {}", alice_shared);
    println!("Bob's Shared Secret: {}", bob_shared);
    println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
    Ok(())
}
