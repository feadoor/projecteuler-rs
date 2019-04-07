//! [Problem 115 (Counting block combinations II)](https://projecteuler.net/problem=115)
//!
//! # Solution detail
//!
//! There is an easy-to-calculate recurrence relation that describes the number of ways that a row
//! of length `n` can be filled by blocks of length at least `m`. Let `aₙ` be the number of ways of
//! doing so. We have several choices for how to fill the row:
//!
//!   - The row can be filled with one really long brick
//!   - The leftmost position can be unoccupied and we have `aₙ₋₁` choices for the rest of the row
//!   - The leftmost position can be taken by a brick of length `k` and there are `aₙ₋ₖ₋₁` choices
//!     for the rest of the row.
//!
//! This leads to the recurrence relation `aₙ = 1 + aₙ₋₁ + aₙ₋ₘ₋₁ + aₙ₋ₘ₋₂ + ... + a₀`
//!
//! Comparing this expression with the equivalent one obtained for `aₙ₋₁`, we get the simplified
//! formula `aₙ = 2aₙ₋₁ - aₙ₋₂ + aₙ₋ₘ₋₁`.
//!
//! This can be used to calculate values `aₙ` in linear time, knowing the base cases
//! `a₀ = a₁ = ... = aₘ₋₁ = 1, aₘ = 2`.

use std::collections::VecDeque;

use projecteuler_rs::problem;

/// A structure capable of iterating over the number of ways of tiling a row of length `n` with
/// bricks of length at least `m`.
struct BricksRecurrenceIterator {
    m: usize,
    values: VecDeque<usize>,
}

impl BricksRecurrenceIterator {

    /// Construct a new `BricksRecurrenceIterator` that calculates the values of the recurrence for
    /// bricks of length at least `m`.
    fn new(m: usize) -> BricksRecurrenceIterator {
        let mut values = VecDeque::from(vec![1; m]); values.push_back(2);
        BricksRecurrenceIterator { m, values }
    }
}

impl Iterator for BricksRecurrenceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.values.pop_front().unwrap();
        self.values.push_back(2 * self.values[self.m - 1] - self.values[self.m - 2] + next_value);
        Some(next_value)
    }
}

/// Find the first `n` for which the number of ways of filling a row of length `n` with bricks at
/// least `m` units long exceeds the given threshold.
fn solve(m: usize, target: usize) -> usize {
    BricksRecurrenceIterator::new(m).position(|x| x > target).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50, 1_000_000).to_string()
}

problem!(answer, "168");
