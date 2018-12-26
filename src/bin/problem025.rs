//! [Problem 25 (1000-digit Fibonacci number)](https://projecteuler.net/problem=25)
//!
//! # Solution detail
//!
//! We don't actually need to calculate any Fibonacci numbers for this, which is a relief, since
//! it's nice to avoid dealing with 1000-digit numbers if we can help it. Since F<sub>n</sub> is
//! the closest integer to `(phi ^ n) / √5`, we just need to find the first n for which:
//!
//! `n * log_10(ϕ) > 1000 + log_10(√5) - 1`

use projecteuler_rs::problem;

/// Find the index of the first Fibonacci number with at least n digits.
fn solve(n: u64) -> u64 {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let rhs = n as f64 - 1.0 + 5f64.sqrt().log(10.0);
    (rhs / phi.log(10.0)).ceil() as u64
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "4782");
