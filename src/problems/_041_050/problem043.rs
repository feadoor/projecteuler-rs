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

use utils::search::{DepthFirstTree, Pruning};

/// The name of the problem.
pub const NAME: &'static str = "Problem 43";
/// A description of the problem.
pub const DESC: &'static str = "Sub-string divisibility";

/// A description of a step that can be taken in the search tree.
struct SubstringTreeStep {
    next_digit: u64,
}

/// The information that is held about the current state during the tree search.
struct SubstringTree {
    /// The current value that is being examined.
    value: u64,
    /// The number of digits in the current value.
    num_digits: usize,
    /// Which digits appear in the current value.
    digits_used: [bool; 10],
}

impl SubstringTree {
    /// Construct a new `SubstringTree`.
    fn new() -> SubstringTree {
        SubstringTree {
            value: 0,
            num_digits: 0,
            digits_used: [false; 10],
        }
    }

    /// Check if the most recent substring condition has been satisfied.
    fn condition_satisfied(&self) -> bool {
        const MODULI: &'static [u64; 7] = &[2, 3, 5, 7, 11, 13, 17];
        self.num_digits < 4 || (self.value % 1000) % MODULI[self.num_digits - 4] == 0
    }
}

/// Search for numbers which satisfy all the substring divisibility conditions, doing a depth-first
/// search by appending one digit at a time.
impl DepthFirstTree for SubstringTree {
    type Step = SubstringTreeStep;
    type Output = u64;

    /// Al possible choices for the next digit - that is, those which have not yet been used.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        (0..10)
            .filter(|&d| !self.digits_used[d as usize])
            .map(|d| Self::Step { next_digit: d })
            .collect()
    }

    /// Check if the most recent condition is satisfied, and prune the tree if not.
    fn should_prune(&mut self) -> Pruning {
        if !self.condition_satisfied() {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    /// Add the next digit to the end of the current value.
    fn apply_step(&mut self, step: &Self::Step) {
        self.value = 10 * self.value + step.next_digit;
        self.num_digits += 1;
        self.digits_used[step.next_digit as usize] = true;
    }

    /// Remove the last digit from the end of the current value.
    fn revert_step(&mut self, step: &Self::Step) {
        self.value /= 10;
        self.num_digits -= 1;
        self.digits_used[step.next_digit as usize] = false;
    }

    /// Output the current value, if it is the right length and satisfies all conditions.
    fn output(&mut self) -> Option<Self::Output> {
        if self.num_digits == 10 {
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the sum of all 10-digit pandigital numbers satisfying the divisibility conditions.
fn solve() -> u64 {
    SubstringTree::new().iter().sum()
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
