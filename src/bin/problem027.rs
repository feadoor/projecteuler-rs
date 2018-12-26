//! [Problem 27 (Quadratic primes)](https://projecteuler.net/problem=27)
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
use projecteuler_rs::problem;

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
fn answer() -> String {
    solve(1_000, 1_000).to_string()
}

problem!(answer, "-59231");
