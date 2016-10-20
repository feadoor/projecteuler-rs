//! [Problem 15 (Lattice paths)](https://projecteuler.net/problem=15)
//!
//! # Problem statement
//!
//! Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
//!
//! How many such routes are there through a 20×20 grid?
//!
//! # Solution detail
//!
//! We can count the number of paths directly. Each path is a series of 40 steps, either right or
//! down, with 20 of each. The path is uniquely determined by choosing the 20 positions of the
//! downward steps within the path. This is equal to the binomial coefficient `40 choose 20`

use number_theory::binom;

/// The name of the problem.
pub const NAME: &'static str = "Problem 15";
/// A description of the problem.
pub const DESC: &'static str = "Lattice paths";

/// Find the `number of paths through a square grid of size `n`.
fn solve(n: u64) -> u64 {
    binom(2 * n, n)
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(20).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem015() {
        assert_eq!(super::answer(), "137846528820");
    }
}
