//! [Problem 30 (Digit fifth powers)](https://projecteuler.net/problem=30)
//!
//! # Problem statement
//!
//! Surprisingly there are only three numbers that can be written as the sum of fourth powers of
//! their digits:
//!
//! 1634 = 1<sup>4</sup> + 6<sup>4</sup> + 3<sup>4</sup> + 4<sup>4</sup>
//!
//! 8208 = 8<sup>4</sup> + 2<sup>4</sup> + 0<sup>4</sup> + 8<sup>4</sup>
//!
//! 9474 = 9<sup>4</sup> + 4<sup>4</sup> + 7<sup>4</sup> + 4<sup>4</sup>
//!
//! As 1 = 1<sup>4</sup> is not a sum it is not included.
//!
//! The sum of these numbers is 1634 + 8208 + 9474 = 19316. Find the sum of all the numbers that
//! can be written as the sum of fifth powers of their digits.
//!
//! # Solution detail
//!
//! We can find the solutions to this problem using a backtracking-style algorithm, building up
//! the solutions from left-to-right one digit at a time. Note that since 7 × 9<sup>5</sup> has
//! only 6 digits, there cannot be any solutions with more than 6 digits. This means that our
//! search tree only has depth 6.
//!
//! We can also prune the search tree before reaching the bottom by calculating the maximum and
//! minimum values that can be obtained by by the sum of fifth powers after extending the number
//! to length 6, and seeing if the number itself can ever fall within that range.
//!
//! For example, no matter what three digits we add to the end of 211, the sum of fifth powers can
//! never be larger than 2<sup>5</sup> + 1 + 1 + 9<sup>5</sup> + 9<sup>5</sup> + 9<sup>5</sup>,
//! which is equal to 177181. On the other hand, the number itself after adding three digits will
//! always be at least 211000, so there cannot possibly be a solution.
//!
//! Finally, remember to subtract 1 at the end, since it is not counted as a solution.

#[macro_use]
extern crate projecteuler_rs;
extern crate utils;

use utils::search::{DepthFirstTree, Pruning};

/// Fifth powers of single digits.
const FIFTH_POWER: &'static [u64; 10] = &[0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

/// A description of a step that can be taken in the search tree.
struct PowerSumTreeStep {
    /// The digit to add onto the end of the current value.
    next_digit: u64,
}

/// The information that is held about the current state during the tree search.
struct PowerSumTree {
    /// The current value being examined.
    value: u64,
    /// The sum of the fifth powers of the digits of the current value.
    power_sum: u64,
    /// The length, in digits, of the current value.
    length: u64,
    /// The number of digits, including leading zeroes, that a solution must contain.
    solution_length: u64,
}

impl PowerSumTree {
    /// Construct a new `PowerSumTree`.
    fn new() -> PowerSumTree {
        PowerSumTree {
            value: 0,
            power_sum: 0,
            length: 0,
            solution_length: 6,
        }
    }

    /// Check if the current state can possibly be extended to a solution.
    fn may_be_extended(&self) -> bool {
        // Calculate the maximum and minimum values taken by the fifth power sum after extending to
        // `solution_length` digits long.
        let min_power_sum = self.power_sum;
        let max_power_sum = self.power_sum + (self.solution_length - self.length) * FIFTH_POWER[9];

        // Calculate the maximum and minimum values taken by the actual number after extending to
        // `solution_length` digits long.
        let mut min_value = self.value;
        let mut max_value = self.value + 1;
        for _ in 0..self.solution_length - self.length {
            min_value *= 10;
            max_value *= 10;
        }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_power_sum < max_value && max_power_sum >= min_value
    }
}

/// Search for numbers which are equal to the sum of the fifth powers of their digits, doing a
/// depth-first search by appending one digit at a time.
impl DepthFirstTree for PowerSumTree {
    type Step = PowerSumTreeStep;
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
        self.power_sum += FIFTH_POWER[step.next_digit as usize];
        self.length += 1;
    }

    /// Remove the last digit from the current value.
    fn revert_step(&mut self, step: &Self::Step) {
        self.value /= 10;
        self.power_sum -= FIFTH_POWER[step.next_digit as usize];
        self.length -= 1;
    }

    /// Check if the power sum is equal to the value, and output the value if so.
    fn output(&mut self) -> Option<Self::Output> {
        if self.length == self.solution_length && self.value == self.power_sum {
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the sum of the numbers which are equal to the sum of the fifth powers of their digits.
fn solve() -> u64 {
    PowerSumTree::new().into_iter().sum::<u64>() - 1
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "443839");
