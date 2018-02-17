//! Simple functions relating to modular arithmetic.

use internals::{_mod_add, _mod_sub, _mod_mul, _mod_inverse, _normalise};

/// Calculate a + b with respect to the given modulus.
///
/// Depends on the modulus being at most 2^63 - 1 - that is, `2 * modulus` must not overflow.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::mod_add;
///
/// assert_eq!(mod_add(5, 7, 11), 1);
/// assert_eq!(mod_add(2, 100_000_000_000_006, 100_000_000_000_007), 1);
/// ```
#[inline(always)]
pub fn mod_add(a: u64, b: u64, m: u64) -> u64 {
    _mod_add(_normalise(a, m), _normalise(b, m), m)
}

/// Calculate a - b with respect to the given modulus.
///
/// Depends on the modulus being at most 2^63 - 1 - that is, `2 * modulus` must not overflow.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::mod_sub;
///
/// assert_eq!(mod_sub(5, 7, 11), 9);
/// assert_eq!(mod_sub(2, 100_000_000_000_006, 100_000_000_000_007), 3);
/// ```
#[inline(always)]
pub fn mod_sub(a: u64, b: u64, m: u64) -> u64 {
    _mod_sub(_normalise(a, m), _normalise(b, m), m)
}

/// Calcuate a * b with respect to the given modulus, without overflowing for large moduli. Uses
/// a repeated-doubling algorithm, also known as Russian Peasant multiplication.
///
/// Depends on the modulus being at most 2^63 - 1 - that is, `2 * modulus` must not overflow.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::mod_mul;
///
/// assert_eq!(mod_mul(2, 8, 15), 1);
/// assert_eq!(mod_mul(853_467, 21_660_421_200_929, 100_000_000_000_007), 54701091976795);
/// ```
#[inline(always)]
pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    _mod_mul(_normalise(a, m), _normalise(b, m), m)
}

/// Calculate `base ^ exp` with respect to the given modulus.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::mod_exp;
///
/// assert_eq!(mod_exp(2, 4, 17), 16);
/// assert_eq!(mod_exp(2, 4, 15), 1);
/// assert_eq!(mod_exp(73, 101, 991), 456);
/// ```
#[inline(always)]
pub fn mod_exp(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut answer = 1;
    let mut worker = _normalise(base, modulus);
    while exp != 0 {
        if exp & 1 == 1 {
            answer = _mod_mul(answer, worker, modulus);
        }
        exp >>= 1;
        if exp != 0 { worker = _mod_mul(worker, worker, modulus) };
    }

    answer
}

/// Calculates the inverse of `a` with respect to modulus `m`, if it exists.
///
/// Uses an adaptation of the extended Euclidean algorithm, modified to avoid using
/// signed integers.
///
/// # Examples
///
/// ```
/// use modular_arithmetic::mod_inverse;
///
/// assert_eq!(mod_inverse(1, 7), Some(1));
/// assert_eq!(mod_inverse(2, 7), Some(4));
/// assert_eq!(mod_inverse(3, 7), Some(5));
/// assert_eq!(mod_inverse(4, 7), Some(2));
/// assert_eq!(mod_inverse(5, 7), Some(3));
/// assert_eq!(mod_inverse(6, 7), Some(6));
/// ```
#[inline(always)]
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    _mod_inverse(a, m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        for base in vec![2, 3, 5, 1001, u64::max_value()] {
            for modulus in vec![11, 17, 23, u32::max_value() as u64] {
                let mut exp = 1;
                let mod_base = base % modulus as u64;
                let mut true_ans = mod_base;

                while exp < 100 {
                    assert_eq!(mod_exp(base, exp, modulus), true_ans);
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
                let inv = mod_inverse(res, modulus).unwrap();
                assert_eq!((res * inv) % modulus, 1);
            }
        }
    }
}