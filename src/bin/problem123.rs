//! [Problem 123 (Prime square remainders)](https://projecteuler.net/problem=123)
//!
//! # Solution detail
//!
//! By a simple binomial expansion (see problem 120), the remainder in question is:
//!
//!  - `2npₙ` for odd `n`
//!  - `2` for even `n`
//!
//! The nth prime is at least `n`, so this means that the remainder (for odd `n`) is at least `2n²`.
//! That means we only need to generate the first `k` primes, where `k` is such that `2k²` exceeds
//! the sought threshold.
//!
//! We can then perform a binary search on `n` to find the least `n` for which `2npₙ` is large
//! enough, and add 1 if that `n` happens to be even.

use number_theory::integer_sqrt;
use primesieve::Sieve;
use projecteuler_rs::problem;
use search::binary_search;

/// Find the smallest `n` so that the remainder exceeds the given threshold.
fn solve(threshold: u64) -> u64 {
    let number_of_primes = (integer_sqrt(threshold / 2) + 1) as usize;
    let primes: Vec<_> = Sieve::to_n_primes(number_of_primes).iter().collect();

    let remainder = |n| 2 * n * primes[n as usize - 1];
    match binary_search(&remainder, threshold + 1) {
        x if x % 2 == 0 => x + 1,
        x => x,
    }
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000_000_000).to_string()
}

problem!(answer, "21035");
