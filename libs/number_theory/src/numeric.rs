//! Functions relating to basic numeric operations.

use modular_arithmetic::{mod_mul, mod_exp};
use numeric_traits::{Algebraic, DivRem, Saturating, Checked};

/// Returns the greatest common divisor of two positive integers, computed with Euclid's algorithm.
///
/// # Examples
///
/// ```
/// use number_theory::gcd;
///
/// assert_eq!(gcd(89, 55), 1);
/// assert_eq!(gcd(1001, 770), 77);
/// ```
pub fn gcd<T: Algebraic + DivRem + Copy>(mut x: T, mut y: T) -> T {
    let mut tmp;
    while y != T::zero() {
        tmp = x % y;
        x = y;
        y = tmp;
    }
    x
}

/// Returns the least common multiple of two positive integers.
///
/// # Examples
///
/// ```
/// use number_theory::lcm;
///
/// assert_eq!(lcm(89, 55), 4895);
/// assert_eq!(lcm(1001, 770), 10010);
/// ```
pub fn lcm<T: Algebraic + DivRem + Copy>(x: T, y: T) -> T {
    x * (y / gcd(x, y))
}

/// Returns the largest integer not greater than the square root of `n`.
///
/// # Examples
///
/// ```
/// use number_theory::integer_sqrt;
///
/// assert_eq!(integer_sqrt(15u64), 3);
/// assert_eq!(integer_sqrt(16u64), 4);
/// assert_eq!(integer_sqrt(17u64), 4);
///
/// assert_eq!(integer_sqrt(24u64), 4);
/// assert_eq!(integer_sqrt(25u64), 5);
/// ```
pub fn integer_sqrt<T: Algebraic + Saturating + PartialOrd<T> + Into<u64> + From<u64> + Copy>(n: T) -> T {

    let mut sqrt: T = From::from((n.into() as f64).sqrt().floor() as u64);
    while sqrt > T::zero() && sqrt.saturating_mul(sqrt) > n {
        sqrt = sqrt - T::one();
    }
    while (sqrt + T::one()).saturating_mul(sqrt + T::one()) <= n {
        sqrt = sqrt + T::one();
    }

    sqrt
}

/// Returns whether or not the given number is a square.
///
/// # Examples
///
/// ```
/// use number_theory::is_square;
///
/// assert!(is_square(1u64));
/// assert!(!is_square(2u64));
/// assert!(!is_square(3u64));
/// assert!(is_square(4u64));
/// assert!(!is_square(5u64));
/// assert!(!is_square(6u64));
/// ```
pub fn is_square<T: Algebraic + Saturating + PartialOrd<T> + Into<u64> + From<u64> + Copy>(n: T) -> bool {
    let sqrt = integer_sqrt(n);
    n == sqrt * sqrt
}

/// Returns the value of the binomial coefficient `m` choose `n`.
///
/// # Examples
///
/// ```
/// use number_theory::binom;
///
/// assert_eq!(binom(5, 0), 1);
/// assert_eq!(binom(5, 1), 5);
/// assert_eq!(binom(5, 2), 10);
/// assert_eq!(binom(5, 3), 10);
/// assert_eq!(binom(5, 4), 5);
/// assert_eq!(binom(5, 5), 1);
/// ```
pub fn binom<T: Algebraic + DivRem + PartialOrd<T> + Copy>(m: T, mut n: T) -> T {
    // Deal with easy cases, and make n the smaller of the two choices for n.
    if n > m { return T::zero(); }
    if n > m - n { n = m - n; }

    // Calculate the answer iteratively.
    let mut ans = T::one();
    let mut k = T::one();
    while k <= n {
        ans = ans * (m - k + T::one()) / k;
        k = k + T::one();
    }
    ans
}

/// Returns the value of `x` to the power of `y`, using exponentiation by repeated squaring.
///
/// # Examples
///
/// ```
/// use number_theory::pow;
///
/// assert_eq!(pow(2, 0), 1);
/// assert_eq!(pow(2, 1), 2);
/// assert_eq!(pow(2, 2), 4);
/// assert_eq!(pow(2, 3), 8);
/// assert_eq!(pow(2, 4), 16);
///
/// assert_eq!(pow(13, 7), 62_748_517);
/// ```
pub fn pow<T: Algebraic + Copy>(mut x: T, mut y: u64) -> T {

    // Set up somewhere to hold the final answer.
    let mut ans = T::one();

    // Use the repeated squaring algorithm.
    while y != 0 {
        if y & 1 == 1 {
            ans = ans * x;
        }
        y >>= 1;
        if y != 0 { x = x * x; }
    }

    ans
}

/// Returns the value of `x` to the power of `y`, using exponentiation by repeated squaring.
/// Returns `None` if overflow occurs.
///
/// # Examples
///
/// ```
/// use number_theory::checked_pow;
///
/// assert_eq!(checked_pow(2u32, 4), Some(16));
/// assert_eq!(checked_pow(2u32, 33), None);
/// ```
pub fn checked_pow<T: Algebraic + Checked + Copy>(x: T, mut y: u64) -> Option<T> {

    // Set up somewhere to hold the final answer.
    let mut ans = T::one();
    let mut worker = Some(x);

    // Use the repeated squaring algorithm.
    while y != 0 {
        if y & 1 == 1 {
            if let Some(w) = worker {
                match ans.checked_mul(w) {
                    Some(res) => { ans = res; },
                    None => { return None; },
                }
            } else {
                return None;
            }
        }
        y >>= 1;
        if y != 0 { worker = worker.and_then(|x| x.checked_mul(x)); }
    }

    Some(ans)
}

/// Test if `n` is prime, using the deterministic version of the Miller-Rabin primality test.
///
/// # Examples
///
/// ```
/// use number_theory::is_prime;
///
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(3), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(5), true);
///
/// assert_eq!(is_prime(22_801_763_487), false);
/// assert_eq!(is_prime(22_801_763_489), true);
/// assert_eq!(is_prime(22_801_763_491), false);
/// ```
pub fn is_prime(n: u64) -> bool {

    // Use a small set of known-good-enough witnesses for each `n`.
    const WITNESSES: &'static [(u64, &'static [u64])] = &[
        (2_046, &[2]),
        (1_373_652, &[2, 3]),
        (9_080_190, &[31, 73]),
        (25_326_000, &[2, 3, 5]),
        (4_759_123_140, &[2, 7, 61]),
        (1_112_004_669_632, &[2, 13, 23, 1662803]),
        (2_152_302_898_746, &[2, 3, 5, 7, 11]),
        (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
        (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
        (3_825_123_056_546_413_051, &[2, 3, 5, 7, 11, 13, 17, 19, 23]),
        (0xFFFF_FFFF_FFFF_FFFF, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]),
    ];

    // Deal with special cases first
    if n == 1 { return false; }
    if n % 2 == 0 { return n == 2; }

    // Decompose n into a power of 2 and an odd part.
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 { d /= 2; s += 1; }

    // Decide which witnesses to use for this `n`
    let witnesses = WITNESSES.iter().find(|&&(hi, _)| n <= hi).map(|&(_, ws)| ws).unwrap();

    // Loop over all witnesses are test primality
    'outer: for &w in witnesses.iter() {
        let mut power = mod_exp(w, d, n);
        if power == 1 || power == n - 1 { continue 'outer; }
        for _ in 0..s {
            power = mod_mul(power, power, n);
            if power == 1 { return false; }
            else if power == n - 1 { continue 'outer; }
        }
        return false;
    }

    // If nothing witnesses to the compositeness of n, then it is prime.
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let test_cases = vec![(0, 0, 0),
                              (5, 0, 5),
                              (123, 0, 123),
                              (0, 34, 34),
                              (4, 4, 4),
                              (13, 13, 13),
                              (2345, 72, 1),
                              (1406700, 164115, 23445),
                              (1368, 339, 3),
                              (55534, 434334, 2),
                              (30315475, 24440870, 31415)];

        for (x, y, result) in test_cases {
            assert_eq!(gcd(x, y), result);
        }
    }

    #[test]
    fn test_lcm() {
        let test_cases: Vec<(u64, u64, u64)> = vec![(5, 0, 0),
                                                    (123, 0, 0),
                                                    (0, 34, 0),
                                                    (4, 4, 4),
                                                    (13, 13, 13),
                                                    (2345, 72, 168840),
                                                    (1406700, 164115, 9846900),
                                                    (1368, 339, 154584),
                                                    (55534, 434334, 12060152178),
                                                    (30315475, 24440870, 23585439550)];

        for (x, y, result) in test_cases {
            assert_eq!(lcm(x, y), result);
        }
    }

    #[test]
    fn test_integer_sqrt() {
        let check = |x: u64| {
            // Calculate the integer square root.
            let sqrt = integer_sqrt(x);

            // Check that sqrt * sqrt does not exceed x.
            match sqrt.checked_mul(sqrt) {
                None => return false,
                Some(y) if y > x => return false,
                _ => {}
            }

            // Check that (sqrt + 1) * (sqrt + 1) is bigger than x.
            if let Some(y) = (sqrt + 1).checked_mul(sqrt + 1) {
                if y <= x {
                    return false;
                }
            }

            true
        };

        for x in 0..1000 {
            assert!(check(x));
        }

        for x in u64::max_value() - 1000..u64::max_value() {
            assert!(check(x));
        }
    }

    #[test]
    fn test_is_square() {
        for n in 0u64..100 {
            assert!(is_square(n * n));
        }

        for n in 2u64..100 {
            assert!(!is_square(n * n - 1));
            assert!(!is_square(n * n + 1));
        }
    }

    #[test]
    fn test_binom() {
        let results = vec![1, 50, 1225, 19600, 230300, 2118760, 15890700, 99884400, 536878650,
                           2505433700, 10272278170, 37353738800, 121399651100, 354860518600,
                           937845656300, 2250829575120, 4923689695575, 9847379391150,
                           18053528883775, 30405943383200, 47129212243960, 67327446062800,
                           88749815264600, 108043253365600, 121548660036300, 126410606437752,
                           121548660036300, 108043253365600, 88749815264600, 67327446062800,
                           47129212243960, 30405943383200, 18053528883775, 9847379391150,
                           4923689695575, 2250829575120, 937845656300, 354860518600, 121399651100,
                           37353738800, 10272278170, 2505433700, 536878650, 99884400, 15890700,
                           2118760, 230300, 19600, 1225, 50, 1];
        for n in 0u64..51 {
            assert_eq!(binom(50, n), results[n as usize]);
        }
    }

    #[test]
    fn test_pow() {
        let test_cases: Vec<(u64, u64, u64)> = vec![(1, 0, 1),
                                                    (1, 283764, 1),
                                                    (2, 10, 1024),
                                                    (5, 20, 95367431640625),
                                                    (10, 10, 10000000000)];

        for (x, y, result) in test_cases {
            assert_eq!(pow(x, y), result);
        }
    }

    #[test]
    fn test_checked_pow() {
        let test_cases: Vec<(u64, u64, Option<u64>)> = vec![(1, 0, Some(1)),
                                                            (1, 283764, Some(1)),
                                                            (2, 10, Some(1024)),
                                                            (5, 20, Some(95367431640625)),
                                                            (10, 10, Some(10000000000)),
                                                            (2, 65, None),
                                                            (3, 50, None)];

        for (x, y, result) in test_cases {
            assert_eq!(checked_pow(x, y), result);
        }
    }
}