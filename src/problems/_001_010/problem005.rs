//! [Problem 5 (Smallest multiple)](https://projecteuler.net/problem=5)
//!
//! # Solution detail
//!
//! This problem is simply asking for the least common multiple (LCM) of the numbers 1, 2, ..., 20.
//! This can be computed easily as `lcm(lcm(lcm(... (((2, 3), 4), 5), ...)))`

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use number_theory::lcm;


/// Find the smallest number divisible by each of 2, 3, ..., limit.
fn solve(limit: u64) -> u64 {
    (2..limit + 1).fold(1, lcm)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(20).to_string()
}

problem!(answer, "232792560");
