#[allow(dead_code)]
pub fn get_input(message: &str) -> String {
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
        $crate::input::get_input(std::format!("{}", std::format_args!($($arg)*)).as_str())
    };
}
