use crate::calculus::{Derivative, Integral};

use num_traits::cast::FromPrimitive;

#[derive(Debug, Default, Clone)]
pub struct Poly<T> {
    coeff: Vec<T>,
    exp: Vec<T>,
    base: char,
}

// Instantiation functions
#[allow(dead_code)]
impl<
        T: std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Add<Output = T>
            + FromPrimitive
            + std::str::FromStr,
    > Poly<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn new(coeff: Vec<T>, exp: Vec<T>, base: char) -> Option<Self> {
        if coeff.len() == exp.len() {
            Some(Self { coeff, exp, base })
        } else {
            println!("Poly creation error: coeff Vec and exp Vec are different lengths\n");
            None
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let msg = s.trim().replace(' ', "");

        if msg.len() == 0 {
            println!("Given polynomial was empty\n");
            return None;
        }

        // Split the expression into the different terms
        let mut terms: Vec<String> = Vec::new();

        let msg_chars = msg.chars().collect::<Vec<char>>();
        let mut current_term = String::new();

        for i in 0..msg_chars.len() {
            match msg_chars[i] {
                '+' | '-' => {
                    if i > 0 {
                        match msg_chars[i - 1] {
                            '0'..='9' | 'a'..='z' | 'A'..='Z' => {
                                terms.push(current_term.clone());

                                current_term = String::new();
                            }
                            _ => {}
                        }
                    }

                    current_term.push(msg_chars[i]);
                }
                '^' | '0'..='9' | 'a'..='z' | 'A'..='Z' | '.' => {
                    current_term.push(msg_chars[i]);
                }
                _ => {
                    println!("Unexpected character in polynomial\n");
                    return None;
                }
            }
        }

        // Put the last term in the terms Vec
        terms.push(current_term);

        // There are no valid terms
        if terms.len() == 0 {
            println!("No valid terms in given polynomial\n");
            return None;
        }

        let mut coeff_vals = Vec::new();
        let mut exp_vals = Vec::new();
        let mut base: Option<char> = None;

        for term in terms.iter() {
            let mut coeff_sign = true;
            let mut coeff_digits = Vec::new();

            let mut reached_base = false;

            let mut exp_sign = true;
            let mut exp_digits = Vec::new();

            for (i, c) in term.chars().enumerate() {
                match c {
                    '+' | '-' => {
                        if i == 0 {
                            coeff_sign = if c == '+' { true } else { false };
                        } else {
                            exp_sign = if c == '+' { true } else { false };
                        }
                    }
                    '0'..='9' | '.' => {
                        if !reached_base {
                            coeff_digits.push(c);
                        } else {
                            exp_digits.push(c);
                        }
                    }
                    'a'..='z' | 'A'..='Z' => {
                        reached_base = true;

                        if base.is_none() {
                            base = Some(c);
                        } else if base.unwrap() != c {
                            // Ensure only one character is being used for the base
                            println!("Please use the same letter for the base of each term\n");
                            return None;
                        }

                        if coeff_digits.len() == 0 {
                            // No coefficient was written
                            coeff_digits.push('1');
                        }
                    }
                    '^' => {}
                    _ => {
                        // Should never reach this
                        panic!("Unknown character found\n");
                    }
                }
            }

            let coeff_val = coeff_digits
                .into_iter()
                .collect::<String>()
                .parse::<T>()
                .unwrap()
                * if coeff_sign {
                    FromPrimitive::from_isize(1).unwrap()
                } else {
                    FromPrimitive::from_isize(-1).unwrap()
                };

            let exp_val;

            if exp_digits.len() == 0 {
                if reached_base {
                    exp_val = if exp_sign {
                        FromPrimitive::from_isize(1).unwrap()
                    } else {
                        FromPrimitive::from_isize(-1).unwrap()
                    };
                } else {
                    exp_val = FromPrimitive::from_isize(0).unwrap();
                }
            } else {
                exp_val = exp_digits
                    .into_iter()
                    .collect::<String>()
                    .parse::<T>()
                    .unwrap()
                    * if exp_sign {
                        FromPrimitive::from_isize(1).unwrap()
                    } else {
                        FromPrimitive::from_isize(-1).unwrap()
                    };
            }

            coeff_vals.push(coeff_val);
            exp_vals.push(exp_val);
        }

        Self::new(coeff_vals, exp_vals, base.unwrap_or('x'))
    }

    pub fn len(&self) -> usize {
        self.coeff.len()
    }
}

// Helper functions
#[allow(dead_code)]
impl<T: FromPrimitive + std::cmp::PartialOrd + std::str::FromStr + std::marker::Copy> Poly<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Descending exponent order
    pub fn desc(&self) -> Self {
        let mut sorted = self.coeff.iter().zip(self.exp.iter()).collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let (c, e) = sorted.into_iter().unzip();

        Self {
            coeff: c,
            exp: e,
            base: self.base,
        }
    }

    // Ascending exponent order
    pub fn asc(&self) -> Self {
        let mut sorted = self.coeff.iter().zip(self.exp.iter()).collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        let (c, e) = sorted.into_iter().unzip();

        Self {
            coeff: c,
            exp: e,
            base: self.base,
        }
    }

    pub fn degree(&self) -> T {
        self.exp
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone()
    }
}

// Allow Poly to be printed
impl<
        T: FromPrimitive
            + std::cmp::PartialOrd
            + std::str::FromStr
            + std::fmt::Display
            + num_traits::sign::Signed,
    > std::fmt::Display for Poly<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg = String::new();

        for i in 0..self.len() {
            // Positive coefficient
            if self.coeff[i] >= FromPrimitive::from_isize(0).unwrap() {
                // Not the first term to be printed
                if i > 0 {
                    msg.push_str(" + ");
                }
            }
            // Negative coefficient
            else {
                // Not the first term to be printed
                if i > 0 {
                    msg.push_str(" - ");
                } else {
                    msg.push('-')
                }
            }

            if self.exp[i].abs() > FromPrimitive::from_isize(0).unwrap() {
                if self.coeff[i] != FromPrimitive::from_isize(1).unwrap() {
                    msg.push_str(format!("{}{}", self.coeff[i].abs(), self.base).as_str());
                } else {
                    msg.push(self.base);
                }

                if self.exp[i] != FromPrimitive::from_isize(1).unwrap() {
                    // Positive exponent
                    if self.exp[i] > FromPrimitive::from_isize(0).unwrap() {
                        msg.push_str(format!("^{}", self.exp[i]).as_str());
                    }
                    // Negative exponent
                    else {
                        msg.push_str(format!("^(-{})", self.exp[i].abs()).as_str());
                    }
                }
            } else {
                msg.push_str(format!("{}", self.coeff[i].abs()).as_str());
            }
        }

        write!(f, "{}", msg)
    }
}

// Differentiation
impl<
        T: Copy
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Add<Output = T>
            + FromPrimitive
            + std::cmp::PartialOrd
            + std::str::FromStr
            + std::str::FromStr,
    > Derivative for Poly<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn derivative(&self) -> Self {
        let (new_coeff, new_exp) = self
            .coeff
            .clone()
            .iter()
            .zip(self.exp.clone().iter())
            .filter_map(|(&c, &e)| {
                // Remove if new coefficient would be 0
                if e == FromPrimitive::from_isize(0).unwrap() {
                    None
                } else {
                    Some((c * e, e - FromPrimitive::from_isize(1).unwrap()))
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip();

        Self {
            coeff: new_coeff,
            exp: new_exp,
            base: self.base,
        }
    }

    fn nth_derivative(&self, n: usize) -> Self {
        let mut p = self.clone();

        for _i in 0..n {
            p = p.derivative();
        }

        p
    }
}

// Integration
impl<
        T: Copy
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Add<Output = T>
            + FromPrimitive
            + std::cmp::PartialOrd
            + std::str::FromStr
            + std::str::FromStr,
    > Integral for Poly<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn integral(&self) -> Self {
        // TODO add constant C

        let (new_coeff, new_exp) = self
            .coeff
            .clone()
            .iter()
            .zip(self.exp.clone().iter())
            .filter_map(|(&c, &e)| {
                // Remove if new coefficient would be undefined
                if (e + FromPrimitive::from_isize(1).unwrap())
                    == FromPrimitive::from_isize(0).unwrap()
                {
                    None
                } else {
                    Some((
                        c / (e + FromPrimitive::from_isize(1).unwrap()),
                        e + FromPrimitive::from_isize(1).unwrap(),
                    ))
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip();

        Self {
            coeff: new_coeff,
            exp: new_exp,
            base: self.base,
        }
    }

    fn nth_integral(&self, n: usize) -> Self {
        let mut p = self.clone();

        for _i in 0..n {
            p = p.integral();
        }

        p
    }
}
