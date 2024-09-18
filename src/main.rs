fn gcd(a: &i128, b: &i128) -> i128 {
    let mut a = *a;
    let mut b = *b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn gcd_recursive(a: &i128, b: &i128) -> i128 {
    if *b == 0 {
        *a
    } else {
        gcd_recursive(b, &(a % b))
    }
}

fn modular_exponentiation(base: &i128, exponent: &i128, modulus: &i128) -> i128 {
    let mut result = 1i128;
    let mut base_mod = *base % *modulus;
    let mut exp = *exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base_mod) % *modulus;
        }
        base_mod = (base_mod * base_mod) % *modulus;
        exp >>= 1;
    }
    result
}

fn extended_gcd(a: &i128, b: &i128) -> (i128, i128, i128) {
    if *b == 0 {
        return (*a, 1, 0);
    }

    let (gcd, x1, y1) = extended_gcd(b, &(*a % *b));
    let x = y1;
    let y = x1 - (*a / *b) * y1;

    (gcd, x, y)
}

fn main() {
    let ans = gcd(&40, &12);
    let ans1 = gcd_recursive(&40, &12);
    println!("Comparing GCD answers: {ans} compared to {ans1}");

    let a = 98765432123456789;
    let b = 12345678901234567;
    let (gcd, x, y) = extended_gcd(&a, &b);
    println!("GCD: {}, x: {}, y: {}", gcd, x, y);

    let base = 2;
    let exponent = 5;
    let modulus = 13;
    let result = modular_exponentiation(&base, &exponent, &modulus);
    println!("Modular Exponentiation Result: {}", result); // Output: 3
}
