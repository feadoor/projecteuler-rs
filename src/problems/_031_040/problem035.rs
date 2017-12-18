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

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;
extern crate number_theory;
extern crate utils;

use primesieve::Sieve;
use number_theory::{integer_sqrt, pow};
use utils::search::{DepthFirstTree, Pruning};

/// A description of a step that can be taken in the search tree.
struct CircularPrimeTreeStep {
    /// The digit to add onto the end of the current value.
    next_digit: u64,
}

/// The information that is held about the current state during the tree search.
struct CircularPrimeTree {
    /// The current value being examined.
    value: u64,
    /// The digits of the current value.
    digits: Vec<u64>,
    /// The maximum number of digits that we need to consider.
    max_digits: usize,
    /// A sieve to facilitate primality testing during the search.
    sieve: Sieve,
}

impl CircularPrimeTree {
    /// Construct a new `CircularPrimeTree` which will produce circular primes with at most the
    /// specified number of digits.
    fn with_max_digits(max_digits: usize) -> CircularPrimeTree {
        let sieve_limit = integer_sqrt(pow(10, max_digits as u64));
        CircularPrimeTree {
            value: 0,
            digits: Vec::new(),
            max_digits: max_digits,
            sieve: Sieve::to_limit(sieve_limit),
        }
    }

    /// Check if the digits in the current state make a circular prime.
    fn is_circular_prime(&self) -> bool {
        let mut n = self.value;
        if n == 0 {
            return false;
        }

        // Check that each rotation of n is prime
        let power_of_ten = pow(10, self.digits.len() as u64 - 1);
        for digit in self.digits.iter().rev() {
            if !self.sieve.is_prime(n).unwrap() {
                return false;
            } else {
                n = n / 10 + digit * power_of_ten;
            }
        }

        true
    }
}

/// Search for numbers which are circular primes, doing a depth-first search appending one digit
/// at a time.
impl DepthFirstTree for CircularPrimeTree {
    type Step = CircularPrimeTreeStep;
    type Output = u64;

    /// All possible choices of next digit - all circular primes larger than 10 must contain only
    /// the digits 1, 3, 7 and 9.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        [1, 3, 7, 9].iter().map(|&digit| Self::Step { next_digit: digit }).collect()
    }

    /// Prune the tree when we hit the maximum number of digits.
    fn should_prune(&mut self) -> Pruning {
        if self.digits.len() == self.max_digits {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next digit to the end of the current value.
    fn apply_step(&mut self, step: &Self::Step) {
        self.value = 10 * self.value + step.next_digit;
        self.digits.push(step.next_digit);
    }

    /// Remove the last digit from the current value.
    #[allow(unused_variables)]
    fn revert_step(&mut self, step: &Self::Step) {
        self.value /= 10;
        self.digits.pop();
    }

    /// Output the current value, if it is a circular prime.
    fn output(&mut self) -> Option<Self::Output> {
        if self.is_circular_prime() {
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the number of circular primes there are with at most the given number of digits. Remember
/// to add on 2 for the primes 2 and 5 which are not considered otherwise.
fn solve(digits: usize) -> usize {
    CircularPrimeTree::with_max_digits(digits).iter().count() + 2
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(6).to_string()
}

problem!(answer, "55");
