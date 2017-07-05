//! [Problem 34 (Digit factorials)](https://projecteuler.net/problem=34)
//!
//! # Problem statement
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//!
//! Note: as 1! = 1 and 2! = 2 are not sums they are not included.
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

use utils::search::{DepthFirstTree, Pruning};

/// The name of the problem.
pub const NAME: &'static str = "Problem 34";
/// A description of the problem.
pub const DESC: &'static str = "Digit factorials";

// The factorials of single digits.
const FACTORIAL: &'static [u64; 10] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

struct FactorialSumTreeStep {
    next_digit: u64,
}

struct FactorialSumTree {
    value: u64,
    factorial_sum: u64,
    length: u64,
    solution_length: u64,
}

impl FactorialSumTree {
    /// Construct a new `FactorialSumTree`.
    fn new() -> FactorialSumTree {
        FactorialSumTree { value: 0, factorial_sum: 0, length: 0, solution_length: 7 }
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

impl DepthFirstTree for FactorialSumTree {
    type Step = FactorialSumTreeStep;
    type Output = u64;

    fn next_steps(&mut self) -> Vec<Self::Step> {
        (0..10).map(|next_digit| Self::Step { next_digit: next_digit }).collect()
    }

    fn should_prune(&mut self) -> Pruning {
        if self.length == self.solution_length {
            Pruning::Below
        } else if !self.may_be_extended() {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    fn apply_step(&mut self, step: &Self::Step) {
        self.value = 10 * self.value + step.next_digit;
        self.factorial_sum = match self.value {
            0 => 0,
            _ => self.factorial_sum + FACTORIAL[step.next_digit as usize]
        };
        self.length += 1;
    }

    fn revert_step(&mut self, step: &Self::Step) {
        self.factorial_sum = match self.value {
            0 => 0,
            _ => self.factorial_sum - FACTORIAL[step.next_digit as usize],
        };
        self.value /= 10;
        self.length -= 1;
    }

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
    FactorialSumTree::new().iter().sum::<u64>() - 3
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem034() {
        assert_eq!(super::answer(), "40730");
    }
}
