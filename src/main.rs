#[allow(dead_code)]
fn get_input(message: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();

    print!("{}", message);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    s.trim().to_string()
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => {
        $crate::get_input(std::format!("{}", std::format_args!($($arg)*)).as_str())
    };
}

mod poly;

use poly::Poly;

fn main() {
    // input!("{}", "Please enter a polynomial: ").as_str()

    let p = Poly::from_str("10x^3 + 9x^2 + 5");

    println!(
        "You made a degree {} polynomial:\n\t{}",
        p.degree(),
        p.desc()
    );
}
