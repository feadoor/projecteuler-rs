//! [Problem 87 (Prime power triples)](https://projecteuler.net/problem=87)
//!
//! # Solution detail
//!
//! Brute-force is good enough here! Simply loop through the fourth powers, cubes and squares of
//! primes and take their sums, stopping whenever the intermediate result is too large.

use number_theory::integer_sqrt;
use primesieve::Sieve;
use projecteuler_rs::problem;
use std::collections::HashSet;
use std::iter::repeat;

/// Raises n to the given power
fn power(n: u64, pow: usize) -> u64 {
    repeat(n).take(pow).product()
}

/// Find the number of values less than the given limit which are the sum of a square, a cube and a
/// fourth power of a prime.
fn solve(limit: u64) -> usize {
    let primes = Sieve::to_limit(integer_sqrt(limit) + 1);
    let powers_below = |n, k| primes.iter().map(move |p| power(p, n)).take_while(move |&x| x < k);

    powers_below(4, limit).flat_map(|p4|
        powers_below(3, limit - p4).flat_map(move |q3|
            powers_below(2, limit - p4 - q3).map(move |r2|
                p4 + q3 + r2
            )
        )
    ).collect::<HashSet<u64>>().len()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50_000_000).to_string()
}

problem!(answer, "1097343");
