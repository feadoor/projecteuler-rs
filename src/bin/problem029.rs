//! [Problem 29 (Distinct powers)](https://projecteuler.net/problem=29)
//!
//! # Solution detail
//!
//! We can determine the total number of distinct terms without needing to calculate the actual
//! results - by instead calculating the prime factorisations of the results.
//!
//! For each pair of `a` and `b`, compute the prime factorisation of `a`, multiply each exponent
//! by `b`, and store the result in a `HashSet`. The number of elements in the `HashSet` at the
//! end of it all is the answer.

use std::collections::HashSet;
use itertools::Itertools;
use number_theory::integer_sqrt;
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the number of distinct values a^b for 2 ≤ a ≤ `a_lim` and 2 ≤ b ≤ `b_lim`.
fn solve(a_lim: u64, b_lim: u64) -> usize {
    // Somewhere to store the factorisations of the results.
    let mut results = HashSet::<Vec<(u64, u64)>>::new();

    // Sieve for enough primes to be able to factorise each possible value of a.
    let sieve = Sieve::to_limit(integer_sqrt(a_lim));

    // Map each pair of a and b to the prime factorisation of a, b
    let result_factors = (2..a_lim + 1)
        .map(|a| sieve.factorise(a).unwrap())
        .cartesian_product(2..b_lim + 1)
        .map(|(factors, b)| factors.iter().map(|&(base, exp)| (base, b * exp)).collect());

    // Store all the factorisations in the HashSet and return its size
    for factors in result_factors {
        results.insert(factors);
    }
    results.len()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100, 100).to_string()
}

problem!(answer, "9183");
