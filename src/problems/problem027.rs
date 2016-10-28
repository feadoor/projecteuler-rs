//! [Problem 27 (Quadratic primes)](https://projecteuler.net/problem=27)
//!
//! # Problem statement
//!
//! Euler discovered the remarkable quadratic formula:
//!
//! `n² + n + 41`
//!
//! It turns out that the formula will produce 40 primes for the consecutive integer values
//! 0 ≤ n ≤ 39. However, when n = 40, the value 40<sup>2</sup> + 40 + 41 = 40(40+1) + 41 is
//! divisible by 41, and certainly when n = 41, the value 41<sup>2</sup> + 41 + 41 is clearly
//! divisible by 41.
//!
//! The incredible formula `n² − 79n + 1601` was discovered, which produces 80 primes for the
//! consecutive values `0 ≤ n ≤ 79`. The product of the coefficients, −79 and 1601, is −126479.
//!
//! Considering quadratics of the form:
//!
//! `n² + an + b`, where `|a| < 1000` and `|b| ≤ 1000`
//!
//! Find the product of the coefficients, `a` and `b`, for the quadratic expression that produces
//! the maximum number of primes for consecutive values of `n`, starting with `n = 0`.
//!
//! # Solution detail
//!
//! There's nothing more clever to do than simply iterate over the values of `a` and `b` and check,
//! for each pair, how many values of `n` produce successive primes.
//!
//! In order to do this, we will need to be able to test numbers for primality. Since `n = b` will
//! always produce a non-prime, the largest number we need to be able to check for primality is
//! `n = b_max * (b_max + a_max + 1)`, so we can sieve primes up to the square root of that limit
//! once and reuse the same sieve for all primality testing.
//!
//! Finally, we only need to check the values of `b` for which `f(0) = b` is itself prime - this
//! saves some meaningless iteration over `a` for such values of `b`.

use itertools::Itertools;

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 27";
/// A description of the problem.
pub const DESC: &'static str = "Quadratic primes";

/// Find the number of primes produced by n^2 + an + b for successive values n = 0, 1, 2, ...
fn num_primes(a: i64, b: i64, sieve: &Sieve) -> usize {
    (0..)
        .map(|n| n * (n + a) + b)
        .take_while(|&x| x > 0 && sieve.is_prime(x as u64).unwrap())
        .count()
}

/// Find the product of the values of a, b below the given limits which produce the longest
/// sequence of successive primes.
fn solve(a_lim: u64, b_lim: u64) -> i64 {

    // Sieve for enough primes to test all the produced values for primality.
    let sieve = Sieve::to_limit(integer_sqrt(b_lim * (b_lim + a_lim + 1)));

    // Only considering prime values of b, find the pair with the longest produced sequence.
    let (best_b, best_a) = sieve.iter()
        .take_while(|&p| p <= b_lim)
        .cartesian_product(-(a_lim as i64 - 1)..a_lim as i64)
        .max_by_key(|&(b, a)| num_primes(a, b as i64, &sieve))
        .unwrap();

    best_a * best_b as i64
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1_000, 1_000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem027() {
        assert_eq!(super::answer(), "-59231");
    }
}
