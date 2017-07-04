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

use utils::search::DepthFirstTree;

/// The name of the problem.
pub const NAME: &'static str = "Problem 30";
/// A description of the problem.
pub const DESC: &'static str = "Digit fifth powers";

/// Fifth powers of single digits.
const FIFTH_POWER: &'static [u64; 10] = &[0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

/// A structure holding information about a node in the search tree.
struct PowerSumTreeNode {
    value: u64,
    power_sum: u64,
    length: u64,
}

struct PowerSumTree {
    solution_length: u64,
}

impl PowerSumTree {
    /// Construct a new `PowerSumTree`.
    fn new() -> PowerSumTree {
        PowerSumTree { solution_length: 6 }
    }

    /// Check if the given node can possibly be extended to a solution.
    fn may_be_extended(&self, node: &PowerSumTreeNode) -> bool {
        // Calculate the maximum and minimum values taken by the fifth power sum after extending to
        // `solution_length` digits long.
        let min_power_sum = node.power_sum;
        let max_power_sum = node.power_sum + (self.solution_length - node.length) * FIFTH_POWER[9];

        // Calculate the maximum and minimum values taken by the actual number after extending to
        // `solution_length` digits long.
        let mut min_value = node.value;
        let mut max_value = node.value + 1;
        for _ in 0..self.solution_length - node.length {
            min_value *= 10;
            max_value *= 10;
        }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_power_sum < max_value && max_power_sum >= min_value
    }
}

impl DepthFirstTree for PowerSumTree {
    type Node = PowerSumTreeNode;

    fn roots(&self) -> Vec<Self::Node> {
        vec![Self::Node { value: 0, power_sum: 0, length: 0 }]
    }

    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node> {
        (0..10).map(|next_digit| {
            let next_value = 10 * node.value + next_digit;
            let next_power_sum = node.power_sum + FIFTH_POWER[next_digit as usize];
            let next_length = node.length + 1;
            Self::Node { value: next_value, power_sum: next_power_sum, length: next_length }
        }).collect()
    }

    fn should_prune(&mut self, node: &Self::Node) -> bool {
        node.length == self.solution_length || !self.may_be_extended(node)
    }

    fn accept(&mut self, node: &Self::Node) -> bool {
        node.length == self.solution_length && node.value == node.power_sum
    }
}

/// Find the sum of the numbers which are equal to the sum of the fifth powers of their digits.
fn solve() -> u64 {
    PowerSumTree::new().iter().map(|node| node.value).sum::<u64>() - 1
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem030() {
        assert_eq!(super::answer(), "443839");
    }
}
