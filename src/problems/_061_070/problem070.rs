//! [Problem 70 (Totient permutation)](https://projecteuler.net/problem=70)
//!
//! # Solution detail
//!
//! The product formula for ϕ(n) tells us that n / ϕ(n) is equal to the product, over all primes p
//! dividing n, of the quantity p / (p - 1).
//!
//! This quantity is always greater than 1, and is decreasing as p gets larger, so n / ϕ(n) is
//! minimised when n is the product of a small number of large primes - that is, when n is itself
//! prime, or when n is a square of a prime, or a semiprime composed of two close-together primes...
//!
//! It is easy to check that when n is prime, ϕ(n) = n - 1 is never a permutation of n, so we should
//! restrict our search to n the product of two (not necessarily distinct) primes. This can be done
//! quickly by:
//!
//!  - Pre-sieving the primes up to n / 2
//!  - Checking, for n = p * q for each pair p, q, whether ϕ(n) is a permutation of n
//!  - Keeping track of the smallest ratio n / ϕ(n) for solutions as they are found.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;

use std::f64;
use primesieve::Sieve;

/// Determine whether the two given numbers are permutations of one another.
fn are_permutations(m: u64, n: u64) -> bool {

    fn sorted_digits(mut num: u64) -> Vec<u8> {
        let mut digits = Vec::new();
        while num > 0 {
            digits.push((num % 10) as u8);
            num /= 10;
        }
        digits.sort();
        digits
    }

    sorted_digits(m) == sorted_digits(n)
}

/// Find the value of n not exceeding the given limit for which ϕ(n) is a permutation of n and
///  the ratio n / ϕ(n) is minimised.
fn solve(limit: u64) -> u64 {
    let sieve = Sieve::to_limit((limit + 1) / 2);
    let primes: Vec<_> = sieve.iter().collect();

    let mut best_n = 1;
    let mut best_ratio = f64::INFINITY;

    for (idx, p) in primes.iter().enumerate().take_while(|&(_, p)| p * p <= limit) {
        for q in primes[idx..].iter() {
            let n = p * q; if n > limit { break; }
            let phi = (p - 1) * (q - 1);
            let ratio = (n as f64) / (phi as f64);

            if ratio < best_ratio && are_permutations(n, phi) {
                best_n = n;
                best_ratio = ratio;
            }
        }
    }

    best_n
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000_000).to_string()
}

problem!(answer, "8319823");
