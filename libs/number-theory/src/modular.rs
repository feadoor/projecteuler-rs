//! Functions relating in some way to modular arithmetic.

use std::mem::swap;

use misc::{bezout, gcd};

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
pub fn modexp(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut answer = 1;
    let mut worker = base % modulus;
    while exp != 0 {
        if exp & 1 == 1 {
            answer = safe_mod_mul(answer, worker, modulus);
        }
        exp >>= 1;
        if exp != 0 { worker = safe_mod_mul(worker, worker, modulus) };
    }

    answer
}

/// Calcuate a * b with respect to the given modulus, without overflowing for large moduli. Uses
/// a repeated-doubling algorithm, also known as Russian Peasant multiplication.
///
/// Depends on the modulus being at most 2^63 - 1, that is 2 * modulus must not overflow.
///
/// # Examples
///
/// ```
/// use number_theory::safe_mod_mul;
///
/// assert_eq!(safe_mod_mul(2, 8, 15), 1);
/// assert_eq!(safe_mod_mul(853_467, 21_660_421_200_929, 100_000_000_000_007), 54701091976795);
/// ```
pub fn safe_mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    match a.checked_mul(b) {
        Some(x) => x % m,
        None => {
            if a > b { swap(&mut a, &mut b); }
            b = b % m;
            let bits_per_loop = m.leading_zeros();
            let mask = (1 << bits_per_loop) - 1;
            let mut result = 0;

            while a > 0 {
                if a & mask != 0 { result = (result + b * (a & mask)) % m; }
                a >>= bits_per_loop;
                b = (b << bits_per_loop) % m;
            }

            result
        }
    }
}

/// Calculate a value `x` such that `a'x == 1 (mod m')` where `a', m'` are calculated from `a, m`
/// by dividing out their greatest common divisor.
///
/// # Examples
///
/// ```
/// use number_theory::mod_inverse;
///
/// assert_eq!(mod_inverse(1, 7), 1);
/// assert_eq!(mod_inverse(2, 7), 4);
/// assert_eq!(mod_inverse(3, 7), 5);
/// assert_eq!(mod_inverse(4, 7), 2);
/// assert_eq!(mod_inverse(5, 7), 3);
/// assert_eq!(mod_inverse(6, 7), 6);
/// ```
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (s, _) = bezout(a, m);
    let mut ans = s % m;
    if ans < 0 {
        ans += m;
    }
    ans
}

/// A named type for a CRT-style constraint.
type Constraint = (Vec<i64>, i64);

/// Calculates the residue classes that satisfy each of the given constraints.
///
/// Each constraint is in the form of a list of acceptable residue classes, and a modulus.
///
/// # Examples
///
/// ```
/// use number_theory::crt;
///
/// let con_a = (vec![2], 3);
/// let con_b = (vec![3], 5);
/// let con_c = (vec![2], 7);
///
/// assert_eq!(crt(&vec![con_a, con_b, con_c]), (vec![23], 105));
/// ```
pub fn crt(constraints: &[Constraint]) -> Constraint {

    // Define an inner function that will combine two constraints together.
    fn combine(con_a: &Constraint, con_b: &Constraint) -> Constraint {

        // Find the GCD of the two moduli.
        let (mod_a, mod_b) = (con_a.1, con_b.1);
        let g = gcd(mod_a, mod_b);
        let (r, s) = (mod_a / g, mod_b / g);

        // To find x == a (mod gr) and x == b (mod gs) we first find k with
        // a + kgr == b (mod gs). This is k = ((b - a) / g) * r_inv (mod s)
        let mut classes = Vec::new();
        let modulus = g * r * s;
        let r_inv = mod_inverse(r, s);

        for res_a in &con_a.0 {
            for res_b in &con_b.0 {
                if (res_b - res_a) % g == 0 {
                    let mut class = (res_a + (res_b - res_a) * r_inv * r) % modulus;
                    if class < 0 {
                        class += modulus;
                    }
                    classes.push(class);
                }
            }
        }

        (classes, modulus)
    }

    // Combine all the constraints together
    constraints.into_iter().fold((vec![0], 1), |acc, x| combine(&acc, &x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modexp() {
        for base in vec![2, 3, 5, 1001, u64::max_value()] {
            for modulus in vec![11, 17, 23, u32::max_value() as u64] {
                let mut exp = 1;
                let mod_base = base % modulus as u64;
                let mut true_ans = mod_base;

                while exp < 100 {
                    assert_eq!(modexp(base, exp, modulus), true_ans);
                    true_ans = (true_ans * mod_base) % modulus as u64;
                    exp += 1;
                }
            }
        }
    }

    #[test]
    fn test_mod_inverse() {
        for modulus in vec![2, 3, 5, 7, 11, 13, 17] {
            for res in 1..modulus {
                let inv = mod_inverse(res, modulus);
                assert_eq!((res * inv) % modulus, 1);
            }
        }
    }

    #[test]
    fn test_crt() {
        let moduli = vec![3, 5, 7, 11, 13, 17];
        let constraints = moduli
            .iter()
            .map(|&x| (vec![(x - 1) / 2], x))
            .collect::<Vec<Constraint>>();
        let result = crt(&constraints);

        assert_eq!(result, (vec![127627], 255255));

        let moduli = vec![5, 7, 11];
        let constraints = moduli
            .iter()
            .map(|&x| (vec![(x - 1) / 2, (x + 1) / 2], x))
            .collect::<Vec<Constraint>>();
        let mut result = crt(&constraints);
        result.0.sort();

        assert_eq!(result, (vec![17, 38, 137, 192, 193, 248, 347, 368], 385));
    }
}