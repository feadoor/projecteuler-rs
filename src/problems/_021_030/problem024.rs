//! [Problem 24 (Lexicographic permutations)](https://projecteuler.net/problem=24)
//!
//! # Solution detail
//!
//! We can find the `n`th lexicographic permutation of `m` digits using the following iterative
//! algorithm:
//!
//! Find the largest `k` with `k * (m-1)! < n`. Then, the `k`th smallest (counting from 0) of the
//! `m` digits is the first in the permutation. Now take:
//!
//! `n --> n - k * (m-1)!`
//!
//! Then, the remainder of the permutation is the nth lexicographic permutation of the reamining
//! digits, which can be computed iteratively using the same algorithm.

#[macro_use]
extern crate projecteuler_rs;

/// Find the nth lexicographic permutation of [0, 1, ..., m - 1].
fn solve(mut n: usize, m: usize) -> Vec<usize> {
    let mut digits: Vec<usize> = (0..m).collect();
    let mut factorial: usize = (1..m).product();
    let mut permutation = Vec::with_capacity(m);

    while !digits.is_empty() {

        // Find the largest k with n > k * (m - 1)!, then add it to the permutation so far and
        // remove it from the remaining digits.
        let k = (n - 1) / factorial;
        permutation.push(digits.remove(k));

        // Update variables ready for the next iteration of this loop.
        n -= k * factorial;
        if !digits.is_empty() {
            factorial /= digits.len();
        }
    }

    permutation
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let permutation = solve(1_000_000, 10);
    permutation.iter().fold("".to_string(), |acc, x| acc + &x.to_string())
}

problem!(answer, "2783915460");
