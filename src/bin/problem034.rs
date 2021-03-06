//! [Problem 34 (Digit factorials)](https://projecteuler.net/problem=34)
//!
//! # Solution detail
//!
//! We can find the solutions to this problem using a backtracking-style algorithm, building up
//! the solutions from left-to-right one digit at a time. Note that since 8 × 9! has
//! only 7 digits, there cannot be any solutions with more than 7 digits. This means that our
//! search tree only has depth 7.
//!
//! We can also prune the search tree before reaching the bottom by calculating the maximum and
//! minimum values that can be obtained by by the sum of factorials after extending the number
//! to length 7, and seeing if the number itself can ever fall within that range.
//!
//! For example, no matter what three digits we add to the end of 211, the sum of factorials can
//! never be larger than 2! + 1! + 1! + 9! + 9! + 9! + 9!, which is equal to 1451524. On the
//! other hand, the number itself after adding four digits will always be at least 2110000, so
//! there cannot possibly be a solution.
//!
//! Finally, remember to subtract 3 at the end, since 1 and 2 are not counted as solutions.

use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

// The factorials of single digits.
const FACTORIAL: &'static [u64; 10] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

/// A description of a step that can be taken in the search tree.
struct FactorialSumTreeStep {
    /// The digit to add onto the end of the current value.
    next_digit: u64,
}

/// The information that is held about the current state during the tree search.
struct FactorialSumTree {
    /// The current value being examined.
    value: u64,
    /// The sum of the factorials of the digits of the current value.
    factorial_sum: u64,
    /// The length, in digits, of the current value.
    length: u64,
    /// The number of digits, including leading zeroes, that a solution must contain.
    solution_length: u64,
}

impl FactorialSumTree {
    /// Construct a new `FactorialSumTree`.
    fn new() -> FactorialSumTree {
        FactorialSumTree {
            value: 0,
            factorial_sum: 0,
            length: 0,
            solution_length: 7,
        }
    }

    /// Check if the current state can possibly be extended to a solution.
    fn may_be_extended(&self) -> bool {
        // Calculate the maximum and minimum values taken by the factorial sum after extending to
        // `solution_length` digits long.
        let min_factorial_sum = self.factorial_sum;
        let max_factorial_sum = self.factorial_sum + (self.solution_length - self.length) * FACTORIAL[9];

        // Calculate the maximum and minimum values taken by the actual number after extending to
        // `solution_length` digits long.
        let mut min_value = self.value;
        let mut max_value = self.value + 1;
        for _ in 0..self.solution_length - self.length {
            min_value *= 10;
            max_value *= 10;
        }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_factorial_sum < max_value && max_factorial_sum >= min_value
    }
}

/// Search for numbers which are equal to the sum of the factorials of their digits, doing a
/// depth-first search by appending one digit at a time.
impl DepthFirstTree for FactorialSumTree {
    type Step = FactorialSumTreeStep;
    type Output = u64;

    /// Return all possible choices for the next digit to add to the current state.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        (0..10).map(|next_digit| Self::Step { next_digit: next_digit }).collect()
    }

    /// Check if we have reached the maximum depth, and also if we can discount this section of the
    /// tree entirely based on the largest and smallest possible extensions.
    fn should_prune(&mut self) -> Pruning {
        if self.length == self.solution_length {
            Pruning::Below
        } else if !self.may_be_extended() {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    /// Add the next digit to the end of the current value.
    fn apply_step(&mut self, step: &Self::Step) {
        self.value = 10 * self.value + step.next_digit;
        self.factorial_sum = match self.value {
            0 => 0,
            _ => self.factorial_sum + FACTORIAL[step.next_digit as usize],
        };
        self.length += 1;
    }

    /// Remove the last digit from the current value.
    fn revert_step(&mut self, step: &Self::Step) {
        self.factorial_sum = match self.value {
            0 => 0,
            _ => self.factorial_sum - FACTORIAL[step.next_digit as usize],
        };
        self.value /= 10;
        self.length -= 1;
    }

    /// Check if the factorial sum is equal to the value, and output the value if so.
    fn output(&mut self) -> Option<Self::Output> {
        if self.length == self.solution_length && self.value == self.factorial_sum {
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the sum of the numbers which are equal to the sum of the factorials of their digits.
fn solve() -> u64 {
    FactorialSumTree::new().into_iter().sum::<u64>() - 3
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "40730");
