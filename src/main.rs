mod finite_field;
mod polynomial;

fn main() {
    let start = std::time::Instant::now();
    let a_coeffs = vec![1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let b_coeffs = vec![1, 1]; // Big-Endian: x + 1

    let a = polynomial::Polynomial::new(&a_coeffs);
    let b = polynomial::Polynomial::new(&b_coeffs);

    println!("a: {}", a);
    println!("b: {}", b);

    println!("Is a irreducable? {}", a.is_irreducible());
    let (quotient, remainder) = a.div_rem(&b);
    let gcd = a.gcd(&b);
    println!(
        "(Quotient, Remainder) of a/b: ({}, {})",
        quotient, remainder
    ); // Expected: x^2 + 1

    println!("GCD of a, b: {}", gcd);

    // Define the modulus polynomial: x^3 + x + 1
    let modulus_coeffs = vec![1, 0, 0, 0, 1, 1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let modulus = polynomial::Polynomial::new(&modulus_coeffs);

    println!("Modulus: {}", modulus);
    // Define a polynomial: x^2
    let c_coeffs = vec![1, 0, 1, 0, 0, 1, 1]; // Big-Endian: x^2
    let c = polynomial::Polynomial::new(&c_coeffs);
    println!("c: {}", c);

    // Find the inverse of a modulo modulus
    match c.inverse(&modulus) {
        Some(inv_c) => println!("Inverse of c: {}", inv_c), // Expected: x + 1
        None => println!("a has no inverse"),
    }
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
