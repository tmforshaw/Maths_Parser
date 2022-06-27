pub struct Poly {
    coeff: Vec<isize>,
    exp: Vec<isize>,
    base: char,
}

// pub struct ConstPoly<const N: usize> {
//     coeff: [isize; N],
//     exp: [isize; N],
//     base: char,
// }

// impl<const N: usize> ConstPoly<N> {}

#[allow(dead_code)]
impl Poly {
    pub fn new(coeff: Vec<isize>, base: char) -> Self {
        Self {
            coeff: coeff.clone(),
            exp: ((0 as isize)..(coeff.len() as isize))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Wrong size iterator"),
            base,
        }
    }

    fn with_exp(coeff: Vec<isize>, exp: Vec<isize>, base: char) -> Self {
        assert_eq!(coeff.len(), exp.len());

        Self { coeff, exp, base }
    }

    pub fn from_str(s: &str) -> Self {
        let mut msg = s.trim().replace(' ', "");

        // Has no sign at the start
        if !(msg.as_str().starts_with('-') || msg.as_str().starts_with('+')) {
            msg = format!("{}{}", '+', msg)
        }

        // Split the expression into the different terms
        let mut terms: Vec<String> = Vec::new();
        {
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
                    _ => {
                        current_term.push(msg_chars[i]);
                    }
                }
            }

            terms.push(current_term);
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
                    '0'..='9' => {
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
                        } else {
                            assert_eq!(base.unwrap(), c);
                        }

                        if coeff_digits.len() == 0 {
                            // No coefficient was written
                            coeff_digits.push('1');
                        }
                    }
                    '^' => {}
                    _ => panic!("Unknown character"),
                }
            }

            let coeff_val = coeff_digits
                .into_iter()
                .collect::<String>()
                .parse::<isize>()
                .unwrap()
                * if coeff_sign { 1 } else { -1 };

            let mut exp_val = 0;

            if exp_digits.len() == 0 {
                if reached_base {
                    exp_val = if exp_sign { 1 } else { -1 };
                }
            } else {
                exp_val = exp_digits
                    .into_iter()
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap()
                    * if exp_sign { 1 } else { -1 };
            }

            coeff_vals.push(coeff_val);
            exp_vals.push(exp_val);
        }

        Self::with_exp(coeff_vals, exp_vals, base.unwrap())
    }

    pub fn len(&self) -> usize {
        self.coeff.len()
    }

    // Descending exponent order
    pub fn desc(&self) -> Self {
        let mut sorted = self.coeff.iter().zip(self.exp.iter()).collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

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
        sorted.sort_by(|a, b| a.1.cmp(b.1));

        let (c, e) = sorted.into_iter().unzip();

        Self {
            coeff: c,
            exp: e,
            base: self.base,
        }
    }

    pub fn degree(&self) -> isize {
        self.exp.iter().max().unwrap().clone()
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg = String::new();

        for i in 0..self.len() {
            // Positive coefficient
            if self.coeff[i] >= 0 {
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

            if self.exp[i].abs() > 0 {
                if self.coeff[i] != 1 {
                    msg.push_str(format!("{}{}", self.coeff[i].abs(), self.base).as_str());
                } else {
                    msg.push(self.base);
                }

                if self.exp[i] != 1 {
                    // Positive exponent
                    if self.exp[i] > 0 {
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
