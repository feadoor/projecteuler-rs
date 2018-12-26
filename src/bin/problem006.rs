//! [Problem 6 (Sum square difference)](https://projecteuler.net/problem=6)
//!
//! # Solution detail
//!
//! Each of the two quantities in question has a direct formula:
//!
//! 1<sup>2</sup> + 2<sup>2</sup> + ... + n<sup>2</sup> = `n * (n + 1) * (2n + 1) / 6`
//!
//! (1 + 2 + ... + n)<sup>2</sup> = `(n * (n + 1) / 2) ^ 2`
//!
//! The second of these quantites is always larger, so just calculate them directly and take the
//! difference.

use projecteuler_rs::problem;

/// Find the difference between the sum of the squares of the first `n` natural numbers and the
/// square of the sum of the first `n` natural numbers.
fn solve(n: u64) -> u64 {
    let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;
    let sum = n * (n + 1) / 2;
    sum * sum - sum_of_squares
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "25164150");
