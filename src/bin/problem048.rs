//! [Problem 48 (Self powers)](https://projecteuler.net/problem=48)
//!
//! # Solution detail
//!
//! This is simply a case of using, say, a repeated squaring algorithm to calculate the necessary
//! powers modulo 10<sup>10</sup>, and adding up the results.
//!
//! Since the modulus here is larger than 2<sup>32</sup>, care must be taken that no intermediate
//! multiplication ever results in overflow.

use modular_arithmetic::{mod_add, mod_exp};
use projecteuler_rs::problem;

/// Find the last 10 digits of the sum 1^1 + 2^2 + ... + n^n.
fn solve(n: u64) -> u64 {
    let modulus = 10_000_000_000;
    (1..n + 1).map(|x| mod_exp(x, x, modulus)).fold(0, |x, y| mod_add(x, y, modulus))
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1000).to_string()
}

problem!(answer, "9110846700");
