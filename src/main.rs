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
    // let p = Poly::new([1, 2, -3], 'x');

    let p = Poly::<3>::new_from_str("1x^+2 - 10x + 1");

    // println!("{}", p);

    // println!("You typed: {}", input!("{}", "Please enter some text: "));
}
