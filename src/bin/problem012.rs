//! [Problem 12 (Highly divisible triangular number)](https://projecteuler.net/problem=12)
//!
//! # Solution detail
//!
//! We'll iterate up through the triangle numbers in order, and calculate how many divisors each
//! one has, stopping when we find one with 500 divisors.
//!
//! To calculate the number of divisors of the triangle number `T(n) = n * (n+1) / 2 ` we'll need
//! to have some precomputed prime numbers. How many? Well, the number of divisors of `T(n)` is:
//!
//! `d(n) * d((n + 1) / 2)` if `n` is odd.
//!
//! `d(n / 2) * d(n + 1)` if `n` is even.
//!
//! This means that to factorise `T(n)`, we need to know about the primes up to `sqrt(n)`.
//!
//! Finally, we need an upper bound on `n` to know in advance how many primes are required. The
//! product of the first `k` primes has `2^k` divisors, so there is definitely some `n` less than
//! the product of the first 9 primes with at least 500 divisors. That means that we should sieve
//! primes up to the square root of this product.

use number_theory::integer_sqrt;
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the first triangle number to have more than `n` divisors.
fn solve(n: u64) -> u64 {

    // Find an upper bound on how large the first `m` such that `T(m)` has `n` divisors might be.
    let k = (n as f64).log(2.0) as usize;
    let bound = Sieve::to_n_primes(k).iter()
        .take(k)
        .fold(1u64, |prod, p| prod.saturating_mul(p));

    // Sieve for the primes up to the square root of this bound.
    let sieve = Sieve::to_limit(integer_sqrt(bound));

    // Start searching for a triangle number with `n` divisors.
    for m in 1.. {

        let divisors = match m & 1 {
            0 => {
                sieve.number_of_divisors(m / 2).unwrap() *
                sieve.number_of_divisors(m + 1).unwrap()
            }
            _ => {
                sieve.number_of_divisors(m).unwrap() *
                sieve.number_of_divisors((m + 1) / 2).unwrap()
            }
        };

        if divisors > n {
            return m * (m + 1) / 2;
        }
    }

    0
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(500).to_string()
}

problem!(answer, "76576500");
