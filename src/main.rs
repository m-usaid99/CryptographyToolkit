mod polynomial;

fn main() {
    let start = std::time::Instant::now();
    let a_coeffs = vec![1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let b_coeffs = vec![1, 1]; // Big-Endian: x + 1

    let a = polynomial::Polynomial::new(&a_coeffs);
    let b = polynomial::Polynomial::new(&b_coeffs);

    println!("a: {}", a.to_string());
    println!("b: {}", b.to_string());

    let (quotient, remainder) = a.div_rem(&b);
    let gcd = a.gcd(b);
    println!(
        "(Quotient, Remainder) of a/b: ({}, {})",
        quotient.to_string(),
        remainder.to_string()
    ); // Expected: x^2 + 1
    println!("GCD of a, b: {}", gcd.to_string());

    // Define the modulus polynomial: x^3 + x + 1
    let modulus_coeffs = vec![1, 0, 0, 0, 1, 1, 0, 1, 1]; // Big-Endian: x^3 + x + 1
    let modulus = polynomial::Polynomial::new(&modulus_coeffs);

    println!("Modulus: {}", modulus.to_string());
    // Define a polynomial: x^2
    let c_coeffs = vec![1, 0, 1, 0, 0, 1, 1]; // Big-Endian: x^2
    let c = polynomial::Polynomial::new(&c_coeffs);
    println!("c: {}", c.to_string());

    // Find the inverse of a modulo modulus
    match c.inverse(&modulus) {
        Some(inv_c) => println!("Inverse of c: {}", inv_c.to_string()), // Expected: x + 1
        None => println!("a has no inverse"),
    }
    let duration = start.elapsed();
    println!("Time Taken: {:?}", duration);
}
