//! [Problem 64 (Odd period square roots)](https://projecteuler.net/problem=64)
//!
//! # Problem statement
//!
//! All square roots are periodic when written as continued fractions. For
//! example, we can write √23 = [4; (1, 3, 1, 8)] to indicate that the block
//! (1, 3, 1, 8) repeats indefinitely.
//!
//! The first ten continued fraction representations of (irrational) square
//! roots are:
//!
//!   - √2 = [1; (2)], period = 1
//!   - √3 = [1; (1, 2)], period = 2
//!   - √5 = [2; (4)], period = 1
//!   - √6 = [2; (2, 4)], period = 2
//!   - √7 = [2; (1, 1, 1, 4)], period = 4
//!   - √8 = [2; (1, 4)], period = 2
//!   - √10 =[3; (6)], period = 1
//!   - √11 = [3; (3, 6)], period = 2
//!   - √12 = [3; (2, 6)], period = 2
//!   - √13 = [3; (1, 1, 1, 1, 6)], period = 5
//!
//! Exactly four continued fractions, for N ≤ 13, have an odd period.
//!
//! How many continued fractions for N ≤ 10000 have an odd period?
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
