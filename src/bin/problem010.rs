//! [Problem 10 (Summation of primes)](https://projecteuler.net/problem=10)
//!
//! # Solution detail
//!
//! This really is as simple as sieving for the primes below 2000000 and adding them up.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the sum of the primes less than the given limit.
fn solve(limit: u64) -> u64 {
    Sieve::to_limit(limit).iter()
        .take_while(|&x| x < limit)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2_000_000).to_string()
}

problem!(answer, "142913828922");
