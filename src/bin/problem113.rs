//! [Problem 113 (Non-bouncy numbers)](https://projecteuler.net/problem=113)
//!
//! # Solution detail
//!
//! Brute force is clearly not good enough given the size of the problem area. Instead we need a
//! clever formula.
//!
//! Firstly, let's count the increasing numbers that have exactly `d` digits. These correspond to
//! the number of ways that we can choose `d` digits from 1, 2, ..., 9, allowing repeats, and this
//! is given by the formula `binom(d + 8, 8)`.
//!
//! Secondly, decreasing numbers correspond to choosing `d` digits from 9, 8, ..., 1, 0, again
//! allowing repeats, but discounting the all-0 number. This is given by the similar formula
//! `binom(d + 9, 9) - 1`.
//!
//! We have now overcounted, because some numbers are both increasing and decreasing. For each `d`,
//! there are 9 of these, one for each digit 1, 2, ..., 9.
//!
//! Finally, we need to sum these formulae over all `d` - and that has a closed form too.
//!
//! You can easily prove by induction that the sum of `binom(d + k, k)` over `1 ≤ d ≤ n` is given
//! by the formula `binom(n + k + 1, k + 1) - 1`
//!
//! So summing up the formula for increasing numbers, the formula for decreasing numbers and the
//! correction termfor overcounting over `1 ≤ d ≤ n`, the number of non-bouncy numbers with at most
//! `n` digits is given by:
//!
//! `binom(n + 9, 9) + binom(n + 10, 10) - 10n - 2`

use number_theory::binom;
use projecteuler_rs::problem;


/// Find the number of non-bouncy numbers with at most the given number of digits.
fn solve(digits: usize) -> usize {
    binom(digits + 9, 9) + binom(digits + 10, 10) - 10 * digits - 2
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "51161058134250");
