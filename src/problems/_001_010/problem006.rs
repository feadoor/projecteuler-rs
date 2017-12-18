//! [Problem 6 (Sum square difference)](https://projecteuler.net/problem=6)
//!
//! # Problem statement
//!
//! The sum of the squares of the first ten natural numbers is 1<sup>2</sup> + 2<sup>2</sup> + ...
//! + 10<sup>2</sup> = 385
//!
//! The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)<sup>2</sup> =
//! 55<sup>2</sup> = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the
//! square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers
//! and the square of the sum.
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

#[macro_use]
extern crate projecteuler_rs;

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
