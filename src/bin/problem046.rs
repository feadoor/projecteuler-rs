//! [Problem 46 (Goldbach's other conjecture)](https://projecteuler.net/problem=46)
//!
//! # Solution detail
//!
//! It is easy to find all counterexamples to this conjecture up to a given limit. Simply create
//! a list of all sums of primes and double-squares up to the limit, and see if any numbers are
//! missing. This can be done easily, as long as we can generate primes using e.g. a sieve.
//!
//! Solving the problem is then a case of starting at a tentative limit and increasing it until
//! the first counterexample is found.

use primesieve::Sieve;
use projecteuler_rs::problem;
use std::collections::HashSet;

/// Search for numbers violating the conjecture up to the given limit. To do so, we generate a list
/// of all primes up to this limit, calculate all possible values that can be obtained by adding a
/// double-square onto any prime, and see if any are missing.
fn search(limit: u64) -> Option<u64> {
    let sieve = Sieve::to_limit(limit);
    let sums: HashSet<u64> = sieve.iter()
        .flat_map(|prime| (0..).map(move |x| prime + 2 * x * x).take_while(|&sum| sum < limit))
        .collect();
    (1..limit / 2).map(|x| 2 * x + 1).find(|x| !sums.contains(x))
}

/// Find the smallest odd composite that cannot be written as the sum of a prime and twice a square.
fn solve() -> u64 {
    (1..).map(|x| 5000 * x).map(search).filter_map(|x| x).next().unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "5777");
