use std::fmt::Debug;

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

        let msg = s.trim().replace(' ', "");

        println!("{}", msg);

        let base_regex = Regex::new(r"[a-zA-Z]").unwrap();
        let base = base_regex.find(&msg).unwrap().as_str();

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
