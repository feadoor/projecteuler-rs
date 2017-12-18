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

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;
extern crate primesieve;

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// Find the largest prime factor of n.
fn solve(n: u64) -> u64 {

    // Sieve the primes up to the square root of n.
    let sieve = Sieve::to_limit(integer_sqrt(n));
    sieve.factorise(n).unwrap().last().unwrap().0
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(600_851_475_143).to_string()
}

problem!(answer, "6857");