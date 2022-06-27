pub mod calculus;
mod input;
mod poly;

use poly::Poly;

use crate::calculus::{Derivative, Integral};

fn main() {
    let p = Poly::<f32>::from_str(input!("{}", "Please enter a polynomial: ").as_str()).desc();
    // let p = Poly::<f32>::from_str("10x^3 + 9x^2 + 5").desc();

    println!("You input a degree {} polynomial:\n\t{}", p.degree(), p);
    println!();
    println!("The derivative of this is:\n\t{}", p.derivative());

    let n = 2;
    println!();
    println!(
        "The {}th derivative of this is:\n\t{}",
        n,
        p.nth_derivative(n)
    );

    println!();
    println!("The integral of this is:\n\t{}", p.integral());

    let n = 2;
    println!();
    println!("The {}th integral of this is:\n\t{}", n, p.nth_integral(n));
}
