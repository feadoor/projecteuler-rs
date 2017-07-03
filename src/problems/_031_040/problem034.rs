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

use utils::search::{DepthFirstSearcher, DepthFirstNode};

/// The name of the problem.
pub const NAME: &'static str = "Problem 34";
/// A description of the problem.
pub const DESC: &'static str = "Digit factorials";

// A constant, representing the length in digits of any solution.
const SOL_LEN: u64 = 7;

// The factorials of single digits.
const FACTORIAL: &'static [u64; 10] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

/// A structure holding information about a node in the search tree.
struct TreeNode {
    value: u64,
    factorial_sum: u64,
    length: u64,
}

impl TreeNode {

    /// Check if the given node can possibly be extended to a solution.
    fn may_be_extended(&self) -> bool {
        // Calculate the maximum and minimum values taken by the factorial sum after extending to
        // SOL_LEN digits long.
        let min_factorial_sum = self.factorial_sum;
        let max_factorial_sum = self.factorial_sum + (SOL_LEN - self.length) * FACTORIAL[9];

        // Calculate the maximum and minimum values taken by the actual number after extending to
        // SOL_LEN digits long.
        let mut min_value = self.value;
        let mut max_value = self.value + 1;
        for _ in 0..SOL_LEN - self.length {
            min_value *= 10;
            max_value *= 10;
        }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_factorial_sum < max_value && max_factorial_sum >= min_value
    }
}

impl DepthFirstNode for TreeNode {

    fn children(&self) -> Vec<Self> {
        (0..10).map(|next_digit| {
            let next_value = 10 * self.value + next_digit;
            let next_factorial_sum = match next_value {
                0 => 0,
                _ => self.factorial_sum + FACTORIAL[next_digit as usize],
            };
            let next_length = self.length + 1;
            TreeNode { value: next_value, factorial_sum: next_factorial_sum, length: next_length }
        }).collect()
    }

    fn should_prune(&self) -> bool {
        self.length == SOL_LEN || !self.may_be_extended()
    }

    fn accept(&self) -> bool {
        self.length == SOL_LEN && self.value == self.factorial_sum
    }
}

/// Find the sum of the numbers which are equal to the sum of the factorials of their digits.
fn solve() -> u64 {
    let root = TreeNode{ value: 0, factorial_sum: 0, length: 0};
    DepthFirstSearcher::new(root).map(|node| node.value).sum::<u64>() - 3
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
