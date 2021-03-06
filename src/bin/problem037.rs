//! [Problem 37 (Truncatable primes)](https://projecteuler.net/problem=37)
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
use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

/// A description of a step that can be taken in the search tree.
struct TruncatablePrimeTreeStep {
    next_digit: u64,
}

/// The information that is held about the current state during the tree search.
struct TruncatablePrimeTree {
    /// The current value being examined.
    value: u64,
    /// A sieve to facilitate primality testing.
    sieve: Sieve,
}

impl TruncatablePrimeTree {
    /// Construct a new `TruncatablePrimeTree`
    fn new() -> TruncatablePrimeTree {
        TruncatablePrimeTree {
            value: 0,
            sieve: Sieve::to_limit(1000),
        }
    }

    /// Determine if the given value is prime, expanding the sieve if necessary.
    fn is_prime(&mut self, value: u64) -> bool {
        while self.sieve.limit().saturating_mul(self.sieve.limit()) < value {
            self.sieve = Sieve::to_limit(10 * self.sieve.limit());
        }
        self.sieve.is_prime(value).unwrap()
    }

    /// Determine if the current state is a left-truncatable prime, expanding the sieve if necessary.
    fn is_left_truncatable(&mut self) -> bool {
        let mut power = 1;
        while power < self.value {
            power *= 10;
            let value = self.value % power;
            if !self.is_prime(value) {
                return false;
            }
        }
        true
    }
}

/// Search for numbers which are truncatable primes, doing a depth-first search by appending one
/// digit at a time.
impl DepthFirstTree for TruncatablePrimeTree {
    type Step = TruncatablePrimeTreeStep;
    type Output = u64;

    /// All possible choices for the next digit, taking into account that the resulting value must
    /// be prime, and so there are not many valid choices for the final digit.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        let digits = if self.value == 0 {
            vec![2, 3, 5, 7]
        } else {
            vec![1, 3, 7, 9]
        };
        digits.iter().map(|&digit| Self::Step { next_digit: digit }).collect()
    }

    /// Prune the tree above the current value if it is not prime - this way we only examine
    /// right-truncatable primes when traversing the tree.
    fn should_prune(&mut self) -> Pruning {
        let value = self.value;
        if value > 0 && !self.is_prime(value) {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    /// Add the next digit to the end of the current value.
    fn apply_step(&mut self, step: &Self::Step) {
        self.value = 10 * self.value + step.next_digit;
    }

    /// Remove the last digit from the current value.
    #[allow(unused_variables)]
    fn revert_step(&mut self, step: &Self::Step) {
        self.value /= 10;
    }

    /// Output the current value, if it is a truncatable prime.
    fn output(&mut self) -> Option<Self::Output> {
        if self.is_left_truncatable() {
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the sum of all primes which are both left-truncatable and right-truncatable. Remember to
/// subtract the single-digit primes which are not counted.
fn solve() -> u64 {
    TruncatablePrimeTree::new().into_iter().sum::<u64>() - 17
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "748317");
