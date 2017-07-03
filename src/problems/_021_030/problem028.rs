//! [Problem 28 (Number spiral diagonals)](https://projecteuler.net/problem=28)
//!
//! # Problem statement
//!
//! Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is
//! formed as follows:
//!
//! ```text
//! 21 22 23 24 25
//! 20  7  8  9 10
//! 19  6  1  2 11
//! 18  5  4  3 12
//! 17 16 15 14 13
//! ```
//!
//! It can be verified that the sum of the numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same
//! way?
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

/// The name of the problem.
pub const NAME: &'static str = "Problem 28";
/// A description of the problem.
pub const DESC: &'static str = "Number spiral diagonals";

/// Find the sum of the numbers on the diagonals of a spiral grid of size n.
fn solve(n: u64) -> u64 {
    assert_eq!(n % 2, 1);
    (n * (8 + n * (3 + n * 4)) - 9) / 6
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1_001).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem028() {
        assert_eq!(super::answer(), "669171001");
    }
}
