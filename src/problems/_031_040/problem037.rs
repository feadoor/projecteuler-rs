//! [Problem 37 (Truncatable primes)](https://projecteuler.net/problem=37)
//!
//! # Problem statement
//!
//! The number 3797 has an interesting property. Being prime itself, it is possible to continuously
//! remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
//! Similarly we can work from right to left: 3797, 379, 37, and 3.
//!
//! Find the sum of the only eleven primes that are both truncatable from left to right and right
//! to left.
//!
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
//!
//! # Solution detail
//!
//! There is an easy method to generate all the right-truncatable primes: start with the single
//! digit primes 2, 3, 5 and 7 as roots, and do a depth-first traversal of the tree of numbers
//! formed by adding one of the digits 1, 3, 7, 9 to the end of the previous number. We are only
//! interested in the nodes which represent prime numbers, so we prune any branches leading to a
//! non-prime.
//!
//! Then, to solve the problem, simply iterate over the right-truncatable primes, and check which
//! of them are also left-truncatable.
//!
//! Note that we have no a priori upper bound on how large these primes can get, so we will perform
//! primality testing using a prime sieve that starts at some small size and grows when necessary.

use primesieve::Sieve;
use utils::search::DepthFirstTree;

/// The name of the problem.
pub const NAME: &'static str = "Problem 37";
/// A description of the problem.
pub const DESC: &'static str = "Truncatable primes";

/// A structure representing a node in the search tree.
struct TruncatablePrimeTreeNode {
    value: u64,
}

struct TruncatablePrimeTree {
    sieve: Sieve,
}

impl TruncatablePrimeTree {
    /// Construct a new `TruncatablePrimeTree`
    fn new() -> TruncatablePrimeTree {
        TruncatablePrimeTree { sieve: Sieve::to_limit(1000) }
    }

    /// Determine if the given value is prime, expanding the sieve if necessary.
    fn is_prime(&mut self, value: u64)-> bool {
        while self.sieve.limit().saturating_mul(self.sieve.limit()) < value {
            self.sieve = Sieve::to_limit(10 * self.sieve.limit());
        }
        self.sieve.is_prime(value).unwrap()
    }

    /// Determine if the given value if a left-truncatable prime, expanding the sieve if necessary.
    fn is_left_truncatable(&mut self, value: u64) -> bool {
        let mut power = 1;
        while power < value {
            power *= 10;
            if !self.is_prime(value % power) { return false; }
        }
        true
    }
}

impl DepthFirstTree for TruncatablePrimeTree {
    type Node = TruncatablePrimeTreeNode;

    fn roots(&self) -> Vec<Self::Node> {
        [2, 3, 5, 7].iter().map(|&d| Self::Node { value: d }).collect()
    }

    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node> {
        [1, 3, 7, 9].iter()
            .map(|&d| Self::Node { value: 10 * node.value + d })
            .collect()
    }

    fn should_prune(&mut self, node: &Self::Node) -> bool {
        !self.is_prime(node.value)
    }

    fn accept(&mut self, node: &Self::Node) -> bool {
        self.is_left_truncatable(node.value)
    }
}

/// Find the sum of all primes which are both left-truncatable and right-truncatable. Remember to
/// subtract the single-digit primes which are not counted.
fn solve() -> u64 {
    TruncatablePrimeTree::new().iter().map(|node| node.value).sum::<u64>() - 17
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem037() {
        assert_eq!(super::answer(), "748317");
    }
}
