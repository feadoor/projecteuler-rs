//! [Problem 43 (Sub-string divisibility)](https://projecteuler.net/problem=43)
//!
//! # Problem statement
//!
//! The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
//! digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility
//! property.
//!
//! Let d<sub>1</sub> be the 1st digit, d<sub>2</sub> be the 2nd digit, and so on. In this way, we
//! note the following:
//!
//! <ul>
//! <li>d<sub>2</sub>d</sub>3</sub>d<sub>4</sub>=406 is divisible by 2</li>
//! <li>d<sub>3</sub>d</sub>4</sub>d<sub>5</sub>=063 is divisible by 3</li>
//! <li>d<sub>4</sub>d</sub>5</sub>d<sub>6</sub>=635 is divisible by 5</li>
//! <li>d<sub>5</sub>d</sub>6</sub>d<sub>7</sub>=357 is divisible by 7</li>
//! <li>d<sub>6</sub>d</sub>7</sub>d<sub>8</sub>=572 is divisible by 11</li>
//! <li>d<sub>7</sub>d</sub>8</sub>d<sub>9</sub>=728 is divisible by 13</li>
//! <li>d<sub>8</sub>d</sub>9</sub>d<sub>10</sub>=289 is divisible by 17</li>
//! </ul>
//!
//! Find the sum of all 0 to 9 pandigital numbers with this property.
//!
//! # Solution detail
//!
//! The easiest way to solve this problem is by using a depth-first search, building up the number
//! one digit at a time, and pruning branches whenever one of the divisibility conditions is not
//! satisfied.
//!
//! In other words, we consider all those choices for the first four digits which are divisible
//! by 2, then for each of those, consider all choices for the next digit which meets the condition
//! for divisibility by 3, and so on until the whole number has been built.

use utils::search::DepthFirstTree;

/// The name of the problem.
pub const NAME: &'static str = "Problem 43";
/// A description of the problem.
pub const DESC: &'static str = "Sub-string divisibility";

/// A structure representing a node in the search tree.
struct SubstringTreeNode {
    value: u64,
    num_digits: usize,
    last_digit: u64,
}

struct SubstringTree {
    digits_used: [bool; 10],
}

impl SubstringTree {
    /// Construct a new `SubstringTree`.
    fn new() -> SubstringTree {
        SubstringTree { digits_used: [false; 10] }
    }

    /// Check if the most recent substring condition has been satisfied.
    fn condition_satisfied(&self, value: u64, num_digits: usize) -> bool {
        const MODULI: &'static [u64; 7] = &[2, 3, 5, 7, 11, 13, 17];
        num_digits < 4 || (value % 1000) % MODULI[num_digits - 4] == 0
    }
}

impl DepthFirstTree for SubstringTree {
    type Node = SubstringTreeNode;

    fn roots(&self) -> Vec<Self::Node> {
        vec![Self::Node { value: 0, num_digits: 0, last_digit: 0 }]
    }

    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node> {
        (0..10).filter(|&d| !self.digits_used[d as usize]).filter_map(|d| {
            let next_value = 10 * node.value + d;
            let next_num_digits = node.num_digits + 1;

            if self.condition_satisfied(next_value, next_num_digits) {
                Some(Self::Node { value: next_value, num_digits: next_num_digits, last_digit: d })
            } else {
                None
            }
        }).collect()
    }

    fn update_state_before(&mut self, node: &Self::Node) {
        if node.num_digits > 0 {
            self.digits_used[node.last_digit as usize] = true;
        }
    }

    fn update_state_after(&mut self, node: &Self::Node) {
        self.digits_used[node.last_digit as usize] = false;
    }

    #[allow(unused_variables)]
    fn should_prune(&mut self, node: &Self::Node) -> bool {
        false
    }

    fn accept(&mut self, node: &Self::Node) -> bool {
        node.num_digits == 10
    }
}

/// Find the sum of all 10-digit pandigital numbers satisfying the divisibility conditions.
fn solve() -> u64 {
    SubstringTree::new().iter().map(|node| node.value).sum()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem043() {
        assert_eq!(super::answer(), "16695334890");
    }
}
