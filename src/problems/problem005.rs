//! [Problem 5 (Smallest multiple)](https://projecteuler.net/problem=5)
//!
//! # Problem statement
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
//! remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
//! 20?
//!
//! # Solution detail
//!
//! This problem is simply asking for the least common multiple (LCM) of the numbers 1, 2, ..., 20.
//! This can be computed easily as `lcm(lcm(lcm(... (((2, 3), 4, 5, ...)))`

use number_theory::lcm;

/// The name of the problem.
pub const NAME: &'static str = "Problem 5";
/// A description of the problem.
pub const DESC: &'static str = "Smallest multiple";

/// Find the smallest number divisible by each of 2, 3, ..., limit.
fn solve(limit: u64) -> u64 {
    (2..limit + 1).fold(1, |prod, x| lcm(prod, x))
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(20).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem005() {
        assert_eq!(super::answer(), "232792560");
    }
}
