//! [Problem 35 (Circular primes)](https://projecteuler.net/problem=35)
//!
//! # Problem statement
//!
//! The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and
//! 719, are themselves prime.
//!
//! There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//!
//! How many circular primes are there below one million?
//!
//! # Solution detail
//!
//! First, note that circular primes, other than the single digit primes 2 and 5, must consist
//! entirely of the digits 1, 3, 7 and 9, since all other primes end in these digits.
//!
//! This means that we can simply consider all combinations of these digits, and for each one,
//! check if the corresponding number is a circular prime. We do this prime checking using a sieve
//! which holds enough pre-computed prime numbers to do primality checking for each number we might
//! encounter.

use primesieve::Sieve;
use number_theory::{integer_sqrt, pow};
use utils::search::DepthFirstTree;

/// The name of the problem.
pub const NAME: &'static str = "Problem 35";
/// A description of the problem.
pub const DESC: &'static str = "Circular primes";

/// A structure holding information about a node in the search tree used to find circular primes.
struct CircularPrimeTreeNode {
    value: u64,
    digits: Vec<u64>,
}

struct CircularPrimeTree {
    max_digits: usize,
    sieve: Sieve,
}

impl CircularPrimeTree {
    /// Construct a new `CircularPrimeTree` which will produce circular primes with at most the
    /// specified number of digits.
    fn with_max_digits(max_digits: usize) -> CircularPrimeTree {
        let sieve_limit = integer_sqrt(pow(10, max_digits as u64));
        CircularPrimeTree { max_digits: max_digits, sieve: Sieve::to_limit(sieve_limit) }
    }
}

impl DepthFirstTree for CircularPrimeTree {
    type Node = CircularPrimeTreeNode;

    fn roots(&self) -> Vec<Self::Node> {
        vec![Self::Node { value: 0, digits: Vec::new() }]
    }

    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node> {
        [1, 3, 7, 9].iter().map(|&digit| {
            let next_value = 10 * node.value + digit;
            let mut next_digits = node.digits.clone();
            next_digits.push(digit);
            Self::Node { value: next_value, digits: next_digits }
        }).collect()
    }

    fn should_prune(&mut self, node: &Self::Node) -> bool {
        node.digits.len() >= self.max_digits
    }

    fn accept(&mut self, node: &Self::Node) -> bool {
        let mut n = node.value;
        if n == 0 { return false; }

        // Check that each rotation of n is prime
        let power_of_ten = pow(10, node.digits.len() as u64 - 1);
        for digit in node.digits.iter().rev() {
            if !self.sieve.is_prime(n).unwrap() {
                return false;
            } else {
                n = n / 10 + digit * power_of_ten;
            }
        }

        true
    }
}

/// Find the number of circular primes there are with at most the given number of digits. Remember
/// to add on 2 for the primes 2 and 5 which are not considered otherwise.
fn solve(digits: usize) -> usize {
    CircularPrimeTree::with_max_digits(digits).iter().count() + 2
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(6).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem035() {
        assert_eq!(super::answer(), "55");
    }
}
