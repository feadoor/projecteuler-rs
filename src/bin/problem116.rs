//! [Problem 116 (Red, green or blue tiles)](https://projecteuler.net/problem=116)
//!
//! # Solution detail
//!
//! This problem is suited very well to a solution based on a linear recurrence. Let `f(n, m)` be
//! the number of ways of tiling a row of length `n` with tiles of length `m`. We have the
//! following choices:
//!
//!   - Don't put anything in the first position, with `f(n - 1, m)` choices for the rest of the row.
//!
//!   - Put a tile in the first position, with `f(n - m, m)` choices for the rest of the row.
//!
//! This leads to the recurrence `f(n, m) = f(n - 1, m) + f(n - m, m)` with initial values given by
//! `f(0, m) = f(1, m) = ... = f(m - 1, m) = 1`.
//!
//! We seek the value of `f(50, 2) + f(50, 3) + f(50, 4) - 3` - subtracting three because leaving
//! and empty row is not allowed in each case.

use std::collections::VecDeque;

use projecteuler_rs::problem;

/// A structure capable of iterating over the number of ways of tiling a row of length `n` with
/// tiles of length exactly `m`.
struct BricksRecurrenceIterator {
    m: usize,
    values: VecDeque<usize>,
}

impl BricksRecurrenceIterator {

    /// Construct a new `BricksRecurrenceIterator` that calculates the values of the recurrence for
    /// bricks of length `m`.
    fn new(m: usize) -> BricksRecurrenceIterator {
        BricksRecurrenceIterator { m, values: VecDeque::from(vec![1; m]) }
    }
}

impl Iterator for BricksRecurrenceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.values.pop_front().unwrap();
        self.values.push_back(self.values[self.m - 2] + next_value);
        Some(next_value)
    }
}

/// Find the number of ways of tiling a row of length `n` with red, green or blue bricks.
fn solve(n: usize) -> usize {
    (2..5).map(|m| BricksRecurrenceIterator::new(m).nth(n).unwrap()).sum::<usize>() - 3
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50).to_string()
}

problem!(answer, "20492570929");
