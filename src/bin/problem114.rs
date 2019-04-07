//! [Problem 114 (Counting block combinations I)](https://projecteuler.net/problem=114)
//!
//! # Solution detail
//!
//! There is an easy-to-calculate recurrence relation that describes the number of ways that a row
//! of length `n` can be filled. Let `aₙ` be the number of ways of doing so. We have several choices
//! for how to fill the row:
//!
//!   - The row can be filled with one really long brick
//!   - The leftmost position can be unoccupied and we have `aₙ₋₁` choices for the rest of the row
//!   - The leftmost position can be taken by a brick of length `k` and there are `aₙ₋ₖ₋₁` choices
//!     for the rest of the row.
//!
//! This leads to the recurrence relation `aₙ = 1 + aₙ₋₁ + aₙ₋₄ + aₙ₋₅ + ... + a₀`
//!
//! Comparing this expression with the equivalent one obtained for `aₙ₋₁`, we get the simplified
//! formula `aₙ = 2aₙ₋₁ - aₙ₋₂ + aₙ₋₄`.
//!
//! This can be used to calculate values `aₙ` in linear time, knowing the base cases
//! `a₀ = a₁ = a₂ = 1, a₃ = 2`.

use std::collections::VecDeque;

use projecteuler_rs::problem;

/// A structure capable of iterating over the number of ways of tiling a row of length `n`.
struct BricksRecurrenceIterator {
    values: VecDeque<usize>,
}

impl BricksRecurrenceIterator {

    /// Construct a new `BricksRecurrenceIterator`
    fn new() -> BricksRecurrenceIterator {
        BricksRecurrenceIterator {
            values: VecDeque::from(vec![1, 1, 1, 2])
        }
    }
}

impl Iterator for BricksRecurrenceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.values.pop_front().unwrap();
        self.values.push_back(2 * self.values[2] - self.values[1] + next_value);
        Some(next_value)
    }
}

/// Find the number of ways of filling a row of length `n` with bricks.
fn solve(n: usize) -> usize {
    BricksRecurrenceIterator::new().nth(n).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50).to_string()
}

problem!(answer, "16475640049");
