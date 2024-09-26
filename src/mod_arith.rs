pub fn binary_gcd(mut a: i128, mut b: i128) -> i128 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    while b != 0 {
        b >>= b.trailing_zeros();

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= a;
    }

    a << shift
}

pub fn extended_binary_gcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    if b == 0 {
        return (a, 1, 0);
    }

    let shift = (a | b).trailing_zeros();

    a >>= a.trailing_zeros();

    let mut x = 1;
    let mut y = 0;
    let mut u = 0;
    let mut v = 1;

    while b != 0 {
        b >>= b.trailing_zeros();

        if a > b {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x, &mut u);
            std::mem::swap(&mut y, &mut v);
        }

        b -= a;
        u -= x;
        v -= y;
    }

    (a << shift, x, y)
}

pub fn modular_inverse(a: i128, m: i128) -> Option<i128> {
    let (gcd, x, _) = extended_binary_gcd(a, m);
    if gcd != 1 {
        return None;
    }
    Some((x % m + m) % m)
}

pub fn modular_exponentiation(base: &i128, exponent: &i128, modulus: &i128) -> i128 {
    if *modulus == 1 {
        return 0; // Any number mod 1 is 0
    }

    let mut result = 1i128;
    let mut base_mod = *base % *modulus;

    // Ensure base_mod is non-negative
    if base_mod < 0 {
        base_mod += modulus;
    }

    let mut exp = *exponent;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base_mod) % *modulus;

            // Ensure result is non-negative
            if result < 0 {
                result += modulus;
            }
        }
        base_mod = (base_mod * base_mod) % *modulus;

        // Ensure base_mod is non-negative
        if base_mod < 0 {
            base_mod += modulus;
        }

        exp >>= 1;
    }

    result
}
