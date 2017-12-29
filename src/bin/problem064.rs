//! [Problem 64 (Odd period square roots)](https://projecteuler.net/problem=64)
//!
//! # Solution detail
//!
//! There is nothing more clever to do here than to just calculate the continued fractions for each
//! of the square roots, and count its period directly.

#[macro_use]
extern crate projecteuler_rs;
extern crate continued_fractions;

use continued_fractions::PeriodicContinuedFraction;

/// Find the number of square roots, up to the square root of the given number,
/// which have an off period in their continued fraction.
fn solve(n: u64) -> usize {
    (2..n + 1)
        .map(PeriodicContinuedFraction::sqrt)
        .filter(|ref cf| cf.period.len() % 2 == 1)
        .count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000).to_string()
}

problem!(answer, "1322");
