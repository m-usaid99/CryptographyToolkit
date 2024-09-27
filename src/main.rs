mod polynomial;
use polynomial::Polynomial;

fn main() {
    let poly1 = Polynomial::new(&[1, 0, 1, 1]);
    println!("{}", poly1);
}
