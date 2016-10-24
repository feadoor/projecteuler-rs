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

use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 10";
/// A description of the problem.
pub const DESC: &'static str = "Summation of primes";

/// Find the sum of the primes less than the given limit.
fn solve(limit: u64) -> u64 {
    Sieve::to_limit(limit)
        .iter()
        .take_while(|&x| x < limit)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(2_000_000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem010() {
        assert_eq!(super::answer(), "142913828922");
    }
}
