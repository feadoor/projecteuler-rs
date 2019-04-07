//! [Problem 117 (Red, green, and blue tiles)](https://projecteuler.net/problem=117)
//!
//! # Solution detail
//!
//! This problem is suited very well to a solution based on a linear recurrence. Let `f(n, m)` be
//! the number of ways of tiling a row of length `n` with tiles of length between `2` and `m`. We
//! have the following choices:
//!
//!   - Put nothing in the first position, with `f(n - 1, m)` choices for the rest of the row.
//!
//!   - Put a tile of length `k` in the first position, with `f(n - k, m)` choices for the rest.
//!
//! This leads to the recurrence `f(n, m) = f(n - 1, m) + f(n - 2, m) + ... + f(n - m, m)`, having
//! the initial values `f(-m + 1, m), f(-m + 2, m), ..., f(-1), 0 = 0, 0, ..., 0, 1`.
//!
//! We seek the value of `f(50, 4)`.

use std::collections::VecDeque;

use projecteuler_rs::problem;

/// A structure capable of iterating over the number of ways of tiling a row of length `n` with
/// tiles of length at most `m`.
struct BricksRecurrenceIterator {
    values: VecDeque<usize>,
}

impl BricksRecurrenceIterator {

    /// Construct a new `BricksRecurrenceIterator` that calculates the values of the recurrence for
    /// bricks of length `m`.
    fn new(m: usize) -> BricksRecurrenceIterator {
        let mut values = VecDeque::from(vec![0; m - 1]); values.push_back(1);
        BricksRecurrenceIterator { values }
    }
}

impl Iterator for BricksRecurrenceIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = *self.values.back().unwrap();
        self.values.push_back(self.values.iter().sum());
        self.values.pop_front();
        Some(next_value)
    }
}

/// Find the number of ways of tiling a row of length `n` with red, green and blue bricks.
fn solve(n: usize) -> usize {
    BricksRecurrenceIterator::new(4).nth(n).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50).to_string()
}

problem!(answer, "100808458960497");
