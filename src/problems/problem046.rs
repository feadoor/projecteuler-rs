//! [Problem 46 (Goldbach's other conjecture)](https://projecteuler.net/problem=46)
//!
//! # Problem statement
//!
//! It was proposed by Christian Goldbach that every odd composite number can be written as the
//! sum of a prime and twice a square.
//!
//! 9 = 7 + 2 × 12
//!
//! 15 = 7 + 2 × 22
//!
//! 21 = 3 + 2 × 32
//!
//! 25 = 7 + 2 × 32
//!
//! 27 = 19 + 2 × 22
//!
//! 33 = 31 + 2 × 12
//!
//!
//! It turns out that the conjecture was false.
//!
//! What is the smallest odd composite that cannot be written as the sum of a prime and twice a
//! square?

use primesieve::Sieve;

use std::collections::HashSet;

/// The name of the problem.
pub const NAME: &'static str = "Problem 46";
/// A description of the problem.
pub const DESC: &'static str = "Goldbach's other conjecture";

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
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem046() {
        assert_eq!(super::answer(), "5777");
    }
}
