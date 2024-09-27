mod finite_field;
mod polynomial;

use polynomial::Polynomial;

fn main() {
    let poly1 = Polynomial::new(&[1, 0, 1, 1]);
    let poly2 = Polynomial::new(&[1, 0, 0, 1]);
    let sum = poly1.add(&poly2);
    println!("{}", sum);
}
