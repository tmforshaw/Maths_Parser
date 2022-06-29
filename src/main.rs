pub mod calculus;
mod input;
mod poly;

use crate::calculus::{Derivative, Integral};
use poly::Poly;

// TODO add command line options and runtime options
// TODO add constant C for integrals

fn main() {
    let p: Poly<f32>;

    // Accept command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        p = Poly::from_str(input!("{}", "Please enter a polynomial: ").as_str())
            .unwrap()
            .desc();
        // p = Poly::<f32>::from_str("10x^3 + 9x^2 + 5").unwrap().desc();
    } else {
        p = Poly::from_str(args[1].as_str()).unwrap().desc();
    }

    println!("You input a degree {} polynomial:\n\t{}", p.degree(), p);
    println!();
    println!("The derivative of this is:\n\t{}", p.derivative());

    let n = 3;
    println!();
    println!(
        "The {}th derivative of this is:\n\t{}",
        n,
        p.nth_derivative(n)
    );

    println!();
    println!("The integral of this is:\n\t{}", p.integral());

    println!();
    println!("The {}th integral of this is:\n\t{}", n, p.nth_integral(n));
}
