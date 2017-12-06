//! [Problem 60 (Prime pair sets)](https://projecteuler.net/problem=60)
//!
//! # Problem statement
//!
//! The primes 3, 7, 109, and 673, are quite remarkable. By taking any two
//! primes and concatenating them in any order the result will always be prime.
//! For example, taking 7 and 109, both 7109 and 1097 are prime. The sum of
//! these four primes, 792, represents the lowest sum for a set of four primes
//! with this property.
//!
//! Find the lowest sum for a set of five primes for which any two primes
//! concatenate to produce another prime.
//!
//! # Solution detail
//!
//! Use a standard depth-first backtracking algorithm, where each branch in the
//! search tree represents adding a single prime to the set.
//!
//! To speed up the search, branches can be pruned early when we know that the
//! resultant sum cannot be smaller than that of some previously-found set of
//! primes.
//!
//! Finally, we can speed up checking by pre-calculating which pairs of primes
//! concatenate to form another prime.

/// The name of the problem.
pub const NAME: &'static str = "Problem 60";
/// A description of the problem.
pub const DESC: &'static str = "Prime pair sets";

use std::collections::{HashSet, HashMap};
use primesieve::Sieve;
use utils::search::{DepthFirstTree, Pruning};

// A structure representing a set of numbers and their sum.
#[derive(Clone)]
struct SumSet {
    /// The numbers belonging to this set.
    numbers: Vec<u64>,
    /// The sum of the numbers in this set.
    sum: u64,
}

impl SumSet {
    /// A new, empty `NumberSet` consisting of no numbers.
    fn new() -> SumSet {
        SumSet {
            numbers: vec![],
            sum: 0,
        }
    }

    /// Add a number to this `SumSet`
    fn add_number(&mut self, number: u64) {
        self.numbers.push(number);
        self.sum += number;
    }

    /// Remove the most recent number from this `SumSet`
    fn remove_number(&mut self) {
        match self.numbers.pop() {
            Some(num) => self.sum -= num,
            None => {},
        }
    }
}

/// A description of a step that can be taken in the search tree.
struct SumSetTreeStep {
    next_number: u64,
}

/// The information that is held about the current state during the tree search.
struct SumSetTree {
    sum_set: SumSet,
    required_size: usize,
    best_sum: u64,
    allowed_pairs: HashMap<u64, HashSet<u64>>,
}

impl SumSetTree {
    /// Construct a new `SumSetTree` which will search for sets of numbers:
    ///
    ///  - Having the given size
    ///  - Containing only allowed pairs of numbers
    fn new(size: usize, allowed_pairs: HashMap<u64, HashSet<u64>>) -> SumSetTree {
        SumSetTree {
            sum_set: SumSet::new(),
            required_size: size,
            best_sum: u64::max_value(),
            allowed_pairs: allowed_pairs,
        }
    }
}

/// Generate allowable sets of the right size, guaranteeing that the final set
/// generated will be the one with the smallest sum.
impl DepthFirstTree for SumSetTree {
    type Step = SumSetTreeStep;
    type Output = SumSet;

    /// All possible choices of the next number to put into the set.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        if self.sum_set.numbers.is_empty() {
            self.allowed_pairs.keys()
                .map(|num| SumSetTreeStep { next_number: *num })
                .collect()
        } else {
            let set = self.allowed_pairs
                .get(self.sum_set.numbers.last().unwrap())
                .unwrap();

            self.sum_set.numbers.iter()
                .map(|num| self.allowed_pairs.get(num).unwrap())
                .fold(set.clone(), |set1, set2| &set1 & &set2)
                .into_iter()
                .map(|num| SumSetTreeStep { next_number: num })
                .collect()
        }
    }

    /// Don't go any deeper in the tree than the required size, and prune when
    /// the sum is too big.
    fn should_prune(&mut self) -> Pruning {
        let largest_number = self.sum_set.numbers.last().unwrap_or(&0);
        let space_left = (self.required_size - self.sum_set.numbers.len()) as u64;

        if self.sum_set.sum + space_left * largest_number > self.best_sum {
            Pruning::Above
        } else if self.sum_set.numbers.len() == self.required_size {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next symbol to the end of the current template.
    fn apply_step(&mut self, step: &Self::Step) {
        self.sum_set.add_number(step.next_number);
    }

    /// Remove the last symbol from the current template.
    fn revert_step(&mut self, _step: &Self::Step) {
        self.sum_set.remove_number();
    }

    /// Output the current template, if it is of the right length.
    fn output(&mut self) -> Option<Self::Output> {
        if self.sum_set.numbers.len() == self.required_size {
            self.best_sum = self.sum_set.sum;
            Some(self.sum_set.clone())
        } else {
            None
        }
    }
}

/// Find the smallest sum of a set of primes such that any pair of primes in
/// the set concatenate, in both orders, to form another prime. Only considers
/// numbers up to the given maximum prime.
fn solve(size_of_set: usize, max_prime: u64) -> u64 {
    let sieve = Sieve::to_limit(10 * max_prime);
    let primes: Vec<_> = sieve.iter().filter(|&p| p <= max_prime).collect();
    let mut prime_pairs = HashMap::new();

    for (ix, &p) in primes.iter().enumerate() {
        prime_pairs.insert(p, HashSet::new());
        for &q in primes[ix + 1..].iter() {
            let concatenation1: u64 = (p.to_string() + &q.to_string()).parse().unwrap();
            let concatenation2: u64 = (q.to_string() + &p.to_string()).parse().unwrap();
            if sieve.is_prime(concatenation1).unwrap() && sieve.is_prime(concatenation2).unwrap() {
                prime_pairs.get_mut(&p).unwrap().insert(q);
            }
        }
    }

    let mut best_set: SumSet = SumSet::new();
    for sum_set in SumSetTree::new(size_of_set, prime_pairs).iter() {
        best_set = sum_set;
    }
    best_set.sum
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(5, 10_000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem060() {
        assert_eq!(super::answer(), "26033");
    }
}
