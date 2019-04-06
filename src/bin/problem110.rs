//! [Problem 110 (Diophantine reciprocals II)](https://projecteuler.net/problem=110)
//!
//! # Solution detail
//!
//! This is the same solution as to Problem 108 - repeated below.
//!
//! Firstly, some rearrangement of the equation is in order. The given equation can equivalently be
//! written in the form `(x - n)(y - n) = n²`.
//!
//! Given that interchanging `x, y` does not count as a new solution, the number of solutions to
//! this equation, and hence to the original equation, is given by `(d(n²) + 1) / 2`, where `d(n²)`
//! is the number of divisors of `n²`.
//!
//! We are therefore searching for the smallest `n` for which `d(n²)` is at least 8000001 (in
//! general, greater than `2 * target` where `target` is the number of solutions we want to be in
//! excess of).
//!
//! This can be done efficiently using a depth-first search through numbers having the following
//! properties:
//!
//!   - They are _smooth_, in the sense that they are divisible exactly by primes 2, 3, ..., `p` for
//!     some prime `p`.
//!
//!   - They are _descending_, in the sense that the exponents of their prime factors are strictly
//!     decreasing as the prime factor increases.
//!
//! Given these two properties, a number is uniquely determined by the sequence of exponents in its
//! prime factorisation, so we can conduct the search on those without worrying about actual primes,
//! and only convert to the actual solution at the end.
//!
//! Given a sequence of exponents, the number of divisors of the square of that number is equal to
//! the product of `2e + 1` over all exponents `e`.
//!
//! It is therefore easy to conduct a depth-first search over all decreasing sequences of exponents,
//! taking those nodes which correspond to a number whose square has enough divisors, and finding,
//! from those nodes, the one which corresponds to the smallest actual number.
//!
//! To spped the search up, the smallest solution so far can be stored as auxiliary data and nodes
//! pruned whenever the value of the node exceeds the smallest solution found so far.

use std::cmp::min;

use number_theory::pow;
use primesieve::Sieve;
use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

/// The state of the search for numbers whose squares have sufficiently many divisors.
struct SquareDivisorsTree {
    /// The target number of divisors for the search
    target: u64,
    /// The smallest solution found so far
    best_solution: u64,
    /// Some small primes, used to turn a sequence of exponents into an actual number
    primes: Vec<u64>,
    /// The sequence of exponents of the current node
    exponents: Vec<u64>,
    /// The numerical value of the current node
    value: u64,
    /// The number of divisors of the square of the current node
    divisors: u64,
}

/// A step that can be taken in the search tree
struct SquareDivisorsTreeStep {
    /// The exponent to add as the next one in the sequence
    exponent: u64,
}

impl SquareDivisorsTree {

    /// Construct a new `SquareDivisorsTree` which will search for numbers whose squares have at
    /// least the given number of divisors.
    fn new(target: u64) -> Self {
        SquareDivisorsTree {
            target: target,
            best_solution: u64::max_value(),
            primes: Sieve::to_n_primes(100).iter().collect(),
            exponents: Vec::new(),
            value: 1,
            divisors: 1,
        }
    }

    /// Get the prime at index `k`, generating more primes if necessary.
    fn prime_at(&mut self, k: usize) -> u64 {
        if self.primes.len() <= k {
            self.primes = Sieve::to_n_primes(2 * k + 1).iter().collect();
        }
        self.primes[k]
    }

    /// Get an upper bound for the next exponent that can be added to the current node.
    fn next_exponent_ub(&mut self) -> u64 {

        // The next exponent should not exceed the current one
        let last_element_upper_bound = match self.exponents.last() {
            Some(e) => *e,
            None => u64::max_value()
        };

        // The next exponent should not be such that the value overflows
        let p = self.prime_at(self.exponents.len());
        let (mut n, mut no_overflow_upper_bound) = (self.value, 0);
        while let Some(val) = n.checked_mul(p) {
            n = val;
            no_overflow_upper_bound += 1;
        }

        min(last_element_upper_bound, no_overflow_upper_bound)
    }
}

impl DepthFirstTree for SquareDivisorsTree {
    type Step = SquareDivisorsTreeStep;
    type Output = u64;

    /// Return all possible choices for the next value to add to the current sequence of exponents.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        (1..self.next_exponent_ub() + 1).map(|e| SquareDivisorsTreeStep { exponent: e}).collect()
    }

    /// Prune the tree if the value of this node is larger than the best solution we have seen.
    fn should_prune(&mut self) -> Pruning {
        if self.value > self.best_solution {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    /// Add the next exponent to the current sequence
    fn apply_step(&mut self, step: &Self::Step) {
        self.value *= pow(self.prime_at(self.exponents.len()), step.exponent);
        self.divisors *= 2 * step.exponent + 1;
        self.exponents.push(step.exponent);
    }

    /// Remove the last digit from the sequence
    fn revert_step(&mut self, step: &Self::Step) {
        self.exponents.pop();
        self.divisors /= 2 * step.exponent + 1;
        self.value /= pow(self.prime_at(self.exponents.len()), step.exponent);
    }

    /// If the number of divisors exceeds the target, then output the solution
    fn output(&mut self) -> Option<Self::Output> {
        if self.divisors > self.target {
            self.best_solution = self.value;
            Some(self.value)
        } else {
            None
        }
    }
}

/// Find the smallest `n` for which the equation has more than `target` solutions.
fn solve(target: u64) -> u64 {
    SquareDivisorsTree::new(2 * target).into_iter().last().unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(4_000_000).to_string()
}

problem!(answer, "9350130049860600");
