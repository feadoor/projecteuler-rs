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

use std::collections::{HashMap, HashSet};
use primesieve::Sieve;
use utils::search::{DepthFirstTree, Pruning};

/// The result of concatenating the numbers `p` and `q`.
fn concat(p: u64, q: u64) -> u64 {
    let mut power_of_ten = 1;
    while power_of_ten <= q {
        power_of_ten *= 10;
    }
    power_of_ten * p + q
}

/// A structure representing a set of primes, and their sum.
#[derive(Clone)]
struct PrimeSet {
    primes: Vec<u64>,
    sum: u64,
}

impl PrimeSet {
    /// Construct a new, initially empty, `PrimeSet`.
    fn new() -> PrimeSet {
        PrimeSet {
            primes: Vec::new(),
            sum: 0,
        }
    }

    /// Add the given prime to this `PrimeSet`.
    fn add(&mut self, p: u64) {
        self.primes.push(p);
        self.sum += p;
    }

    /// Remove the most recent prime from this `PrimeSet`.
    fn remove(&mut self) {
        match self.primes.pop() {
            Some(x) => self.sum -= x,
            None => {},
        }
    }

    /// Get the number of primes currently held in this `PrimeSet`.
    fn len(&self) -> usize {
        self.primes.len()
    }

    /// Get the largest prime held in this `PrimeSet`.
    fn max(&self) -> Option<u64> {
        self.primes.last().map(|p| *p)
    }
}

/// A structure that holds, for each prime, the primes that are able to
/// concatenate with it, in both orders, to form new primes.
struct PrimePairs {
    /// A cache to hold the results of previous checks.
    pairs: HashMap<u64, HashSet<u64>>,
}

impl PrimePairs {

    /// Creates a new `PrimePairs`, checking the given primes and using the
    /// given sieve for primality checking.
    fn new(primes: &[u64], sieve: &Sieve) -> PrimePairs {
        let mut pairs = HashMap::new();
        let can_concatenate = |p, q| sieve.is_prime(concat(p, q)).unwrap() &&
                                     sieve.is_prime(concat(q, p)).unwrap();

        for (ix, &p) in primes.iter().enumerate() {
            pairs.insert(p, HashSet::new());
            for &q in primes[ix + 1..].iter() {
                if can_concatenate(p, q) {
                    pairs.get_mut(&p).unwrap().insert(q);
                }
            }
        }

        PrimePairs {
            pairs: pairs,
        }
    }

    /// Get a `HashSet` containing all the primes `q` that are allowed to
    /// concatenate with the given prime `p` in both directions, to form new
    /// primes.
    fn get_allowed_primes(&self, p: u64) -> Option<&HashSet<u64>> {
        self.pairs.get(&p)
    }
}

/// A structure that can be used for conducting a depth-first search for
/// prime-pair sets.
struct PrimePairTree {
    /// The prime numbers up to the maximum prime to be considered.
    primes: Vec<u64>,
    /// A checker
    prime_pairs: PrimePairs,
    /// The current set of primes that is being examined.
    prime_set: PrimeSet,
    /// The size of prime set that we are looking for.
    required_size: usize,
    /// The smallest sum of any prime set found so far.
    best_sum: u64,
}

impl PrimePairTree {
    /// Constructs a new `PrimePairTree` which will search for prime pair sets
    /// containing the given number of primes, and containing primes no larger
    /// than the given maximum prime.
    fn new(size: usize, max_prime: u64) -> PrimePairTree {
        let sieve = Sieve::to_limit(max_prime);
        let primes: Vec<_> = sieve.iter().take_while(|&p| p <= max_prime).collect();
        let prime_pairs = PrimePairs::new(&primes, &sieve);

        PrimePairTree {
            primes: primes,
            prime_pairs: prime_pairs,
            prime_set: PrimeSet::new(),
            required_size: size,
            best_sum: u64::max_value(),
        }
    }
}

/// A description of a step that can be taken in the search tree.
struct PrimePairTreeStep {
    next_prime: u64,
}

/// Search for allowable prime sets of the right size, guaranteeing that the
/// final set found will be the one with the smallest sum.
impl DepthFirstTree for PrimePairTree {
    type Step = PrimePairTreeStep;
    type Output = PrimeSet;

    /// All possible choices of the next prime to put into the set.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        if self.prime_set.len() == 0 {
            self.primes.iter().map(|&p| PrimePairTreeStep { next_prime: p }).collect()
        } else {
            let largest_prime = self.prime_set.max().unwrap();
            self.prime_pairs.get_allowed_primes(largest_prime).unwrap().iter()
                .filter(|&p| self.prime_set.primes.iter().all(
                    |&q| self.prime_pairs.get_allowed_primes(q).unwrap().contains(p)
                ))
                .map(|&p| PrimePairTreeStep { next_prime: p} )
                .collect()
        }
    }

    /// Don't go any deeper in the tree than the required size, and prune when
    /// the sum is too big.
    fn should_prune(&mut self) -> Pruning {
        let largest_number = self.prime_set.max().unwrap_or(0);
        let space_left = (self.required_size - self.prime_set.len()) as u64;

        if self.prime_set.sum + space_left * largest_number > self.best_sum {
            Pruning::Above
        } else if self.prime_set.len() == self.required_size {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next symbol to the end of the current template.
    fn apply_step(&mut self, step: &Self::Step) {
        self.prime_set.add(step.next_prime);
    }

    /// Remove the last symbol from the current template.
    fn revert_step(&mut self, _step: &Self::Step) {
        self.prime_set.remove();
    }

    /// Output the current template, if it is of the right length.
    fn output(&mut self) -> Option<Self::Output> {
        if self.prime_set.len() == self.required_size {
            self.best_sum = self.prime_set.sum;
            Some(self.prime_set.clone())
        } else {
            None
        }
    }
}


/// Find the smallest sum of a set of primes such that any pair of primes in
/// the set concatenate, in both orders, to form another prime. Only considers
/// numbers up to the given maximum prime.
fn solve(size_of_set: usize, max_prime: u64) -> u64 {
    let mut best_set: Option<PrimeSet> = None;
    for prime_set in PrimePairTree::new(size_of_set, max_prime).iter() {
        best_set = Some(prime_set);
    }
    best_set.unwrap().sum
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
