//! [Problem 7 (10001st prime)](https://projecteuler.net/problem=7)
//!
//! # Solution detail
//!
//! There's nothing more to this than just sieving for the first 10001 primes and pulling out the
//! one we want.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the `n`th prime number.
fn solve(n: usize) -> u64 {
    Sieve::to_n_primes(n).nth_prime(n - 1).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_001).to_string()
}

problem!(answer, "104743");
