use colored::Colorize;

#[allow(dead_code)]
pub fn newton_raphson_sqrt(x: f64) -> f64 {
    // Initial guess as half of x
    let mut g: f64 = x / 2f64;

    // Iterate to get a more precise answer
    for _ in 0..10 {
        g -= (g * g - x) / (2f64 * g);
    }

    g
}

#[allow(dead_code)]
pub fn is_prime(x: u64) -> bool {
    match x {
        0 | 1 => false,
        2 => true,
        _ => {
            // If it is even then it's not prime
            if x % 2 == 0 {
                return false;
            }

            // Check if any odd number between 3 and sqrt(x) (inclusive) are divisors of x
            for i in (3..=(newton_raphson_sqrt(x as f64) as u64)).step_by(2) {
                if x % i == 0 {
                    return false;
                }
            }

            // All divisors checked and none were found
            true
        }
    }
}

#[allow(dead_code)]
pub fn prime_decomposition(x: u64) -> Vec<u64> {
    match x {
        0 | 1 => vec![x],
        2 => vec![2],
        _ => {
            let mut div = x;
            let mut fac: Vec<u64> = Vec::new();

            // If it's even then add the correct amount of twos
            while div % 2 == 0 {
                div /= 2;
                fac.push(2);
            }

            // Iterate through all odd numbers
            for i in (3..=(newton_raphson_sqrt(x as f64) as u64)).step_by(2) {
                // X has been fully decomposed
                if div == 1 {
                    return fac;
                }

                // Keep dividing by divisor until x can no longer be divided
                while div % i == 0 {
                    div /= i;
                    fac.push(i);
                }
            }

            // X was fully decomposed
            if div == 1 {
                return fac;
            }

            // Nothing divides x
            vec![x]
        }
    }
}

#[allow(dead_code)]
pub fn simple_greatest_common_divisor(values: [u64; 2]) -> Option<u64> {
    if values[0] == 0 || values[1] == 0 {
        return None;
    }

    let mut r = values[0] % values[1];
    let mut a = values[1];
    let mut b = r;

    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }

    if a == 0 {
        return None;
    }

    Some(a)
}

#[allow(dead_code)]
pub fn lowest_common_multiple(vals: Vec<u64>) -> u64 {
    let mut factor_sets: Vec<Vec<u64>> = Vec::new();

    for n in vals {
        factor_sets.push(prime_decomposition(n));
    }

    let mut counts: Vec<Vec<(u64, u64)>> = Vec::new();
    let mut unique_factors: Vec<u64> = Vec::new();

    for (i, factor_set) in factor_sets.into_iter().enumerate() {
        counts.push(Vec::new());

        for val in factor_set {
            if let Some(index) = counts[i].iter().position(|&v| v.1 == val) {
                let c = counts[i][index as usize];

                counts[i][index as usize] = (c.0 + 1, c.1);
            } else {
                counts[i].push((1, val));

                if !unique_factors.contains(&val) {
                    unique_factors.push(val);
                }
            }
        }
    }

    let mut superset: Vec<(u64, u64)> = Vec::new();

    for val in unique_factors {
        superset.push(
            *counts
                .iter()
                .flatten()
                .filter(|v| v.1 == val)
                .max_by_key(|v| v.0)
                .unwrap(),
        );
    }

    superset
        .iter()
        .map(|v| v.0.checked_mul(v.1).unwrap())
        .product()
}

#[allow(dead_code)]
pub fn euler_lowest_common_multiple(values: [u64; 2]) -> u64 {
    values[0] * values[1] / simple_greatest_common_divisor(values).unwrap()
}

#[allow(dead_code)]
pub fn get_pi() -> String {
    let mut digit: f64 = 0f64;

    // Pi = SUMk=0 to infinity 16-k [ 4/(8k+1) – 2/(8k+4) – 1/(8k+5) – 1/(8k+6) ]

    for k in 0..100 {
        let eight_k: f64 = 8f64 * (k as f64);

        let mut this_sum = 4f64 / (eight_k + 1f64)
            - 2f64 / (eight_k + 4f64)
            - 1f64 / (eight_k + 5f64)
            - 1f64 / (eight_k + 6f64);

        // Multiply by 16^-k
        for _i in 0..k {
            this_sum /= 16f64;
        }

        digit += this_sum;
    }

    format!("{}", digit)
}

#[allow(dead_code)]
pub fn spigot_pi(num_digits: u32) -> String {
    // Initialise array with a Vec of 2s
    let mut array: Vec<u32> = vec![2; (((num_digits * 10) / 3) + 1) as usize];

    let mut num_nines: u32 = 0;
    let mut predigit: u32 = 0;

    let mut digit_string: String = String::new();

    for i in 0..=num_digits {
        let mut q: u32 = 0;

        // Put into regular form
        for j in (1..=array.len()).rev() {
            let x = 10 * array[j - 1] + q * (j as u32);

            array[j - 1] = x % (2 * (j as u32) - 1);
            q = x / (2 * (j as u32) - 1);
        }

        array[0] = q % 10;
        q /= 10;

        if q == 9 {
            num_nines += 1;
        } else if q == 10 {
            digit_string.push_str(format!("{}{:0<1$}", predigit + 1, num_nines as usize).as_str());

            num_nines = 0;
            predigit = 0;
        } else {
            if i != 0 {
                digit_string.push_str(format!("{}", predigit).as_str());

                if i == 1 {
                    digit_string.push('.');
                }
            }
            predigit = q;

            if num_nines != 0 {
                digit_string.push_str(format!("{:9<0$}", num_nines as usize).as_str());

                num_nines = 0;
            }
        }
    }

    digit_string.push_str(format!("{}", predigit).as_str());

    digit_string
}

pub fn math_funcs_main() {
    let n = 1000000;

    let now = std::time::Instant::now();

    let primes: Vec<u64> = (0..n).filter(|p| is_prime(*p)).collect();

    let elapsed = now.elapsed();

    println!(
        "Time to calculate primes below {}: {}",
        format!("{}", n).green(),
        format!("{:.3?}", elapsed).red()
    );

    println!(
        "There are {} primes below {}",
        format!("{}", primes.len()).red(), //primes.len()
        format!("{}", n).green()
    );

    let sum_primes: u64 = primes.clone().into_iter().sum();

    println!(
        "The sum of all the primes below {} is: {}\nThe square root of this is {}",
        format!("{}", n).green(),
        format!("{}", sum_primes).red(),
        format!("{:.5}", newton_raphson_sqrt(sum_primes as f64)).red()
    );

    println!(
        "Prime decomposition of {} is {}",
        format!("{}", sum_primes).green(),
        format!("{:?}", prime_decomposition(sum_primes)).red()
    );

    // println!("Calculated PI: {}", format!("{}", get_pi()).red());

    // let num_pi_digits = 1000;

    // println!(
    //     "Spigot PI ({} digits): {}",
    //     format!("{}", num_pi_digits).green(),
    //     format!("{}", spigot_pi(num_pi_digits)).red()
    // );

    // println!(
    //     "The LCM of the first {} primes is: {}",
    //     format!("{}", n).green(),
    //     format!("{}", lowest_common_multiple(primes)).red()
    // );
}
