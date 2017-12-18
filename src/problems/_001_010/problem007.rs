//! [Problem 7 (10001st prime)](https://projecteuler.net/problem=7)
//!
//! # Problem statement
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime
//! is 13.
//!
//! What is the 10 001st prime number?
//!
//! # Solution detail
//!
//! There's nothing more to this than just sieving for the first 10001 primes and pulling out the
//! one we want.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;

use primesieve::Sieve;

/// Find the `n`th prime number.
fn solve(n: usize) -> u64 {
    Sieve::to_n_primes(n).nth_prime(n - 1).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_001).to_string()
}

problem!(answer, "104743");
