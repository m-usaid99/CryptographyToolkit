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

    let p_hex = concat!(
        "FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD1",
        "29024E088A67CC74020BBEA63B139B22514A08798E3404DD",
        "EF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245",
        "E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7ED",
        "EE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3D",
        "C2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F",
        "83655D23DCA3AD961C62F356208552BB9ED529077096966D",
        "670C354E4ABC9804F1746C08CA18217C32905E462E36CE3B",
        "E39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9",
        "DE2BCBF6955817183995497CEA956AE515D2261898FA0510",
        "15728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64",
        "ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7",
        "ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6B",
        "F12FFA06D98A0864D87602733EC86A64521F2B18177B200C",
        "BBE117577A615D6C770988C0BAD946E208E24FA074E5AB31",
        "43DB5BFCE0FD108E4B82D120A92108011A723C12A787E6D7",
        "88719A10BDBA5B2699C327186AF4E23C1A946834B6150BDA",
        "2583E9CA2AD44CE8DBBBC2DB04DE8EF92E8EFC141FBECAA6",
        "287C59474E6BC05D99B2964FA090C3A2233BA186515BE7ED",
        "1F612970CEE2D7AFB81BDD762170481CD0069127D5B05AA9",
        "93B4EA988D8FDDC186FFB7DC90A6C08F4DF435C934063199",
        "FFFFFFFFFFFFFFFF"
    );

    // Decode hex to bytes
    let p_bytes = decode(p_hex)?;
    // Convert bytes to BigUint
    let p = BigUint::from_bytes_be(&p_bytes);
    let g = BigUint::from(2u32); // Generator
    println!(
        "Testing Diffie-Hellman with multiplicative group of size {} with generator {}",
        p, g
    );
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
