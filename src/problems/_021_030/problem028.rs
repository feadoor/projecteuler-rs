//! [Problem 28 (Number spiral diagonals)](https://projecteuler.net/problem=28)
//!
//! # Solution detail
//!
//! We can calculate the answer in constant time by doing a bit of bookkeeping to begin with.
//! With the exception of the number 1 in the centre, the numbers at distance `k` along each
//! diagonal follow a predictable pattern:
//!
//! Top-right: `(2k + 1)²`
//!
//! Top-left: `(2k + 1)² - 2k`
//!
//! Bottom-right: `(2k + 1)² - 4k`
//!
//! Bottom-left: `(2k + 1)² - 6k`
//!
//! Adding these all up, the sum of the numbers on the diagonals at distance `k` from the centre
//! is `16k² + 4k + 4`.
//!
//! For a grid of side-length `n`, we should sum this quantity over the range `1 ≤ k ≤ (n - 1) / 2`
//! and then add on 1 for the centre square. Using the explicit formulae for the sum of `k` and the
//! sum of `k²` over a range, this gives the final formula:
//!
//! `(4n³ + 3n² + 8n - 9) / 6`

#[macro_use]
extern crate projecteuler_rs;

/// Find the sum of the numbers on the diagonals of a spiral grid of size n.
fn solve(n: u64) -> u64 {
    assert_eq!(n % 2, 1);
    (n * (8 + n * (3 + n * 4)) - 9) / 6
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_001).to_string()
}

problem!(answer, "669171001");
