//! [Problem 3 (Largest prime factor)](https://projecteuler.net/problem=3)
//!
//! # Problem statement
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?
//!
//! # Solution detail
//!
//! There's not an awful lot to say here! To find the prime factors of a given number, we just need
//! to use trial division by primes up to at most the square root of the given number. Simply sieve
//! for those primes, factorise the number, and return the largest factor.

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 3";
/// A description of the problem.
pub const DESC: &'static str = "Largest prime factor";

/// Find the largest prime factor of n.
fn solve(n: u64) -> Result<u64, ()> {

    // Sieve the primes up to the square root of n.
    let sieve = Sieve::to_limit(integer_sqrt(n));

    // Factorise n and return the largest factor, or an error if the factorisation failed.
    if let Ok(factors) = sieve.factorise(n) {
        if let Some(factor) = factors.last() {
            return Ok(factor.0);
        }
    }

    Err(())
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    if let Ok(ans) = solve(600851475143) {
        ans.to_string()
    } else {
        "Something went wrong!".to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem003() {
        assert_eq!(super::answer(), "6857");
    }
}