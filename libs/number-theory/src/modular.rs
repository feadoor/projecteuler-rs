//! Functions relating in some way to modular arithmetic.

/// Calculate `base ^ exp` with respect to the given modulus, using exponentiation by repeated
/// squaring.
///
/// # Examples
///
/// ```
/// use number_theory::modexp;
///
/// assert_eq!(modexp(2, 4, 17), 16);
/// assert_eq!(modexp(2, 4, 15), 1);
/// assert_eq!(modexp(73, 101, 991), 456);
/// ```
pub fn modexp(base: u64, mut exp: u64, modulus: u32) -> u64 {
    let mut answer = 1;
    let mut worker = base % modulus as u64;
    while exp != 0 {
        if exp & 1 == 1 {
            answer = (answer * worker) % modulus as u64;
        }
        exp >>= 1;
        worker = (worker * worker) % modulus as u64;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modexp() {
        for base in vec![2, 3, 5, 1001, u64::max_value()] {
            for modulus in vec![11, 17, 23, u32::max_value()] {
                let mut exp = 1;
                let mod_base = base % modulus as u64;
                let mut true_ans = mod_base;

                while exp < 100 {
                    println!("{} {} {}", base, exp, modulus);
                    assert_eq!(modexp(base, exp, modulus), true_ans);
                    true_ans = (true_ans * mod_base) % modulus as u64;
                    exp += 1;
                }
            }
        }
    }
}