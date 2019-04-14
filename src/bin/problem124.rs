//! [Problem 124 (Ordered radicals)](https://projecteuler.net/problem=124)
//!
//! # Solution detail
//!
//! We can use a modified Sieve of Eratosthenes to calculate the radical of every number in a range.
//!
//! Start with an array that will hold the radicals of each number. Initially set each entry to be
//! equal to 1. Then iterate up through the array, starting at index 2.
//!
//! Every time an entry `p` is found whose radical value is still 1 (that means it is a prime) then
//! multiply the radical value of all multiples of `p` by `p`. At the end, all radicals will have
//! been computed.
//!
//! Then it's a simple matter of sorting and finding the answer.

use projecteuler_rs::problem;

/// Find the radicals of each number in the range 1 ≤ `n` ≤ `limit`, using a sieve-based algorithm.
fn radicals(limit: usize) -> Vec<usize> {
    let mut result = vec![1; limit + 1];
    for p in 2..limit + 1 {
        if result[p] == 1 {
            for multiple in (p..limit + 1).step_by(p) {
                result[multiple] *= p;
            }
        }
    }
    result
}

/// Find the `k`th element when all numbers 1 ≤ `n` ≤ `limit` are sorted by radical.
fn solve(limit: usize, k: usize) -> usize {
    let radicals = radicals(limit);
    let mut indices: Vec<_> = (1..limit + 1).collect();
    indices.sort_by_key(|&n| radicals[n]);
    indices[k - 1]
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100_000, 10_000).to_string()
}

problem!(answer, "21417");
