pub struct Poly<const N: usize> {
    coeff: [isize; N],
    pow: [isize; N],
    base: char,
}

#[allow(dead_code)]
impl<const N: usize> Poly<N> {
    pub fn new(coeff: [isize; N], base: char) -> Self {
        Self {
            coeff,
            pow: ((0 as isize)..(N as isize))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Wrong size iterator"),
            base,
        }
    }

    fn new_with_pow(coeff: [isize; N], pow: [isize; N], base: char) -> Self {
        Self { coeff, pow, base }
    }

    pub fn new_from_str(s: &str) -> Self {
        use regex::Regex;

        let mut msg = s.trim().replace(' ', "");

        // Has no sign at the start
        if !(msg.as_str().starts_with('-') || msg.as_str().starts_with('+')) {
            msg = format!("{}{}", '+', msg)
        }

        println!("{}", msg);

        // Get the base that is used in the polynomial
        let base_regex = Regex::new(r"[a-zA-Z]").unwrap();
        let base = base_regex
            .find(&msg)
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();

        // Split the expression into the different terms

        let msg_chars = msg.chars().collect::<Vec<char>>();
        let mut terms: Vec<String> = Vec::new();
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

        println!("{:?}", terms);

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
                        assert_eq!(base, c);
                    }
                    _ => {}
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

            println!("{}   {}", coeff_sign, exp_sign);

            println!("{}   {}", coeff_val, exp_val);
        }

        // for i in 0..msg_chars.len() {
        //     match msg_chars[i] {
        //         '+' => {
        //             if !reached_coeff {
        //                 reached_coeff = true;
        //                 coeff_sign = true;
        //             } else if reached_exp {
        //                 exp_sign = true;
        //             } else {
        //                 if term_index > 0 {
        //                     let coeff_value;

        //                     if coeff_digits.len() > 0 {
        //                         coeff_value = coeff_digits
        //                             .iter()
        //                             .collect::<String>()
        //                             .parse::<isize>()
        //                             .unwrap()
        //                             * if coeff_sign { 1 } else { -1 };
        //                     } else {
        //                         coeff_value = if coeff_sign { 1 } else { -1 };
        //                     }

        //                     let exp_value;

        //                     if exp_digits.len() > 0 {
        //                         exp_value = exp_digits
        //                             .iter()
        //                             .collect::<String>()
        //                             .parse::<isize>()
        //                             .unwrap()
        //                             * if exp_sign { 1 } else { -1 };
        //                     } else {
        //                         exp_value = 0;
        //                     }

        //                     terms.push((coeff_value, exp_value));
        //                     term_index += 1;
        //                 }

        //                 reached_coeff = false;
        //                 reached_exp = false;
        //                 reached_base = false;

        //                 coeff_sign = false;
        //                 exp_sign = false;
        //             }
        //         }
        //         '-' => {
        //             if !reached_coeff {
        //                 reached_coeff = true;
        //                 coeff_sign = false;
        //             } else if reached_exp {
        //                 exp_sign = false;
        //             } else {
        //                 if term_index > 0 {
        //                     let coeff_value;

        //                     if coeff_digits.len() > 0 {
        //                         coeff_value = coeff_digits
        //                             .iter()
        //                             .collect::<String>()
        //                             .parse::<isize>()
        //                             .unwrap()
        //                             * if coeff_sign { 1 } else { -1 };
        //                     } else {
        //                         coeff_value = if coeff_sign { 1 } else { -1 };
        //                     }

        //                     let exp_value;

        //                     if exp_digits.len() > 0 {
        //                         exp_value = exp_digits
        //                             .iter()
        //                             .collect::<String>()
        //                             .parse::<isize>()
        //                             .unwrap()
        //                             * if exp_sign { 1 } else { -1 };
        //                     } else {
        //                         exp_value = 0;
        //                     }

        //                     terms.push((coeff_value, exp_value));
        //                     term_index += 1;
        //                 }

        //                 reached_coeff = false;
        //                 reached_exp = false;
        //                 reached_base = false;

        //                 coeff_sign = false;
        //                 exp_sign = false;
        //             }
        //         }
        //         '0'..='9' => {
        //             if reached_coeff {
        //                 coeff_digits.push(msg_chars[i]);
        //             } else if reached_exp || reached_base {
        //                 reached_exp = true;

        //                 exp_digits.push(msg_chars[i]);
        //             } else {
        //                 println!("{}", term_index);
        //                 println!("{}", msg_chars[i]);

        //                 if term_index > 0 {
        //                     println!("{:?}", terms[(term_index - 1) as usize]);
        //                 }
        //                 panic!("Unexpected digit");
        //             }
        //         }
        //         'a'..='z' | 'A'..='Z' => {
        //             assert_eq!(msg_chars[i], base);
        //             reached_base = true;
        //         }
        //         '^' => reached_exp = true,
        //         _ => {}
        //     }
        // }

        // More than one letter is used for the base
        if base_regex.captures_len() != 1 {
            panic!("Polynomial should only consist of one base");
        }

        let seperator = Regex::new(r"[+-]").unwrap();
        let neg_seperator = Regex::new(r"[^+-]").unwrap();

        let signs: String = neg_seperator
            .split(&msg)
            .filter(|x| seperator.is_match(*x))
            .collect::<Vec<_>>()
            .join("");

        let terms = seperator.split(&msg).collect::<Vec<_>>();
        let mut coeff = Vec::<isize>::new();

        for (i, t) in terms.iter().enumerate() {
            println!("{}", t);

            // if(t.len() )

            let b_i = t.find(base);

            match b_i {
                Some(index) => {
                    if index == 0 {
                        coeff.push(1);
                    } else {
                        let coeff_substr = t.chars().take(index).into_iter().collect::<String>();

                        coeff.push(
                            coeff_substr.parse::<isize>().unwrap()
                                * (if signs.chars().nth(i).unwrap() == '+' {
                                    1
                                } else {
                                    -1
                                }),
                        );
                    }
                }
                None => {
                    coeff.push(
                        t.parse::<isize>().unwrap()
                            * (if signs.chars().nth(i).unwrap() == '+' {
                                1
                            } else {
                                -1
                            }),
                    );
                }
            }
        }

        Self::new(
            ((0 as isize)..(N as isize))
                .collect::<Vec<_>>()
                .try_into()
                .expect("Wrong size iterator"),
            'x',
        )
    }
}

impl<const N: usize> std::fmt::Display for Poly<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg = String::new();

        for (i, c) in self.coeff.iter().enumerate().rev() {
            if i == 0 {
                msg.push_str(
                    format!("{}{} ", if *c >= 0 { "+ " } else { "- " }, (*c).abs(),).as_str(),
                );
            } else if i == 1 {
                msg.push_str(
                    format!(
                        "{}{}{} ",
                        if *c >= 0 { "+ " } else { "- " },
                        (*c).abs(),
                        self.base
                    )
                    .as_str(),
                );
            } else if i < N - 1 {
                msg.push_str(
                    format!(
                        "{}{}{}^{} ",
                        if *c >= 0 { "+ " } else { "- " },
                        (*c).abs(),
                        self.base,
                        self.pow[i]
                    )
                    .as_str(),
                );
            } else {
                msg.push_str(
                    format!(
                        "{}{}{}^{} ",
                        if *c >= 0 { "" } else { "-" },
                        (*c).abs(),
                        self.base,
                        self.pow[i]
                    )
                    .as_str(),
                );
            }
        }

        write!(f, "{}", msg)
    }
}
