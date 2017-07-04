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

use utils::search::DepthFirstTree;

/// The name of the problem.
pub const NAME: &'static str = "Problem 34";
/// A description of the problem.
pub const DESC: &'static str = "Digit factorials";

// The factorials of single digits.
const FACTORIAL: &'static [u64; 10] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

/// A structure holding information about a node in the search tree.
struct FactorialSumTreeNode {
    value: u64,
    factorial_sum: u64,
    length: u64,
}

struct FactorialSumTree {
    solution_length: u64,
}

impl FactorialSumTree {
    /// Construct a new `FactorialSumTree`.
    fn new() -> FactorialSumTree {
        FactorialSumTree { solution_length: 7 }
    }

    /// Check if the given node can possibly be extended to a solution.
    fn may_be_extended(&self, node: &FactorialSumTreeNode) -> bool {
        // Calculate the maximum and minimum values taken by the factorial sum after extending to
        // `solution_length` digits long.
        let min_factorial_sum = node.factorial_sum;
        let max_factorial_sum = node.factorial_sum + (self.solution_length - node.length) * FACTORIAL[9];

        // Calculate the maximum and minimum values taken by the actual number after extending to
        // `solution_length` digits long.
        let mut min_value = node.value;
        let mut max_value = node.value + 1;
        for _ in 0..self.solution_length - node.length {
            min_value *= 10;
            max_value *= 10;
        }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_factorial_sum < max_value && max_factorial_sum >= min_value
    }
}

impl DepthFirstTree for FactorialSumTree {
    type Node = FactorialSumTreeNode;

    fn roots(&self) -> Vec<Self::Node> {
        vec![Self::Node { value: 0, factorial_sum: 0, length: 0 }]
    }

    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node> {
        (0..10).map(|next_digit| {
            let next_value = 10 * node.value + next_digit;
            let next_factorial_sum = match next_value {
                0 => 0,
                _ => node.factorial_sum + FACTORIAL[next_digit as usize],
            };
            let next_length = node.length + 1;
            Self::Node { value: next_value, factorial_sum: next_factorial_sum, length: next_length }
        }).collect()
    }

    fn should_prune(&mut self, node: &Self::Node) -> bool {
        node.length == self.solution_length || !self.may_be_extended(node)
    }

    fn accept(&mut self, node: &Self::Node) -> bool {
        node.length == self.solution_length && node.value == node.factorial_sum
    }
}

/// Find the sum of the numbers which are equal to the sum of the factorials of their digits.
fn solve() -> u64 {
    FactorialSumTree::new().iter().map(|node| node.value).sum::<u64>() - 3
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
