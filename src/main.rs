pub mod calculus;
mod funcs;
mod input;
mod poly;

use crate::{
    calculus::{Derivative, Integral},
    funcs::math_funcs_main,
};
use poly::Poly;

// TODO add command line options and runtime options
// TODO add constant C for integrals

fn main() {
    // Accept command line arguments
    let args: Vec<String> = std::env::args().collect();
    let p: Poly<f32> = if args.len() == 1 {
        Poly::from_str(input!("{}", "Please enter a polynomial: ").as_str())
            .unwrap()
            .desc()
        // p = Poly::<f32>::from_str("10x^3 + 9x^2 + 5").unwrap().desc();
    } else {
        Poly::from_str(args[1].as_str()).unwrap().desc()
    };

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

    println!();
    math_funcs_main();
}
