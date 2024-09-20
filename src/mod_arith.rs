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

fn modular_inverse(a: &i128, m: &i128) -> Option<i128> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        return None;
    }
    Some((x % m + m) % m)
}
