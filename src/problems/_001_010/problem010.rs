//! [Problem 10 (Summation of primes)](https://projecteuler.net/problem=10)
//!
//! # Problem statement
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
//!
//! # Solution detail
//!
//! This really is as simple as sieving for the primes below 2000000 and adding them up.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;

use primesieve::Sieve;

/// Find the sum of the primes less than the given limit.
fn solve(limit: u64) -> u64 {
    Sieve::to_limit(limit)
        .iter()
        .take_while(|&x| x < limit)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2_000_000).to_string()
}

problem!(answer, "142913828922");
