//! [Problem 122 (Efficient exponentiation)](https://projecteuler.net/problem=122)
//!
//! # Solution detail
//!
//! This is, sadly, a problem where a fully correct solution will take much longer to run than we'd
//! ideally be happy with. But we can take a shortcut!
//!
//! Firstly, it is clear by considering the point of view of the exponents that we are looking for
//! optimal [Addition Chains](https://en.wikipedia.org/wiki/Addition_chain). Furthermore, that link
//! explains that, for all `n < 12509`, there is an optimal addition chain which is also a star
//! chain - that is, each addition uses the most recent result.
//!
//! We can therefore vastly cut down the search space by only considering star chains. That makes
//! a traditional depth-first search, where each node consists of the current power and a list of
//! all previous powers, a suitable method for finding all optimal star chains.
//!
//! Keep track, as the search progresses, of the shortest chain found for each power, and sum them
//! after the search is complete.

use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

/// The state of the search for star chains
struct StarChainTree {
    /// The maximum target value to search for chains for
    limit: usize,
    /// The smallest chain seen so far for each target up to the limit
    best_chains: Vec<usize>,
    /// The value that the current star chain reaches
    value: usize,
    /// The values that the current star chain consists of
    members: Vec<usize>,
}

/// A step that can be taken in the search tree
struct StarChainTreeStep {
    /// The value to add onto the current star chain
    addend: usize,
}

impl StarChainTree {

    /// Create a new `StarChainTree` that will search for star chains up to the given limit
    fn new(limit: usize) -> StarChainTree {
        StarChainTree {
            limit: limit,
            best_chains: vec![usize::max_value(); limit + 1],
            value: 1,
            members: vec![1],
        }
    }

    /// The length of the current star chain
    fn length(&self) -> usize {
        self.members.len()
    }
}


impl DepthFirstTree for StarChainTree {
    type Step = StarChainTreeStep;
    type Output = ();

    /// Return all possible choices for the next addend in the chain
    fn next_steps(&mut self) -> Vec<Self::Step> {
        self.members.iter().rev().map(|&addend| StarChainTreeStep { addend }).collect()
    }

    /// Prune the tree if this node cannot lead to an optimal star chain
    fn should_prune(&mut self) -> Pruning {
        if self.value > self.limit || self.length() > self.best_chains[self.value] {
            Pruning::Above
        } else {
            Pruning::None
        }
    }

    /// Add the next addend onto the star chain
    fn apply_step(&mut self, step: &Self::Step) {
        self.value += step.addend;
        self.members.push(self.value);
    }

    /// Remove the most recent addition from the chain
    fn revert_step(&mut self, step: &Self::Step) {
        self.members.pop();
        self.value -= step.addend;
    }

    /// Update the optimal chain lengths
    fn output(&mut self) -> Option<Self::Output> {
        if self.members.len() < self.best_chains[self.value] {
            self.best_chains[self.value] = self.members.len();
        }
        None
    }
}

/// Find the total length of all optimal star chains with targets up to the given limit.
fn solve(limit: usize) -> usize {
    let star_chain_tree = StarChainTree::new(limit).run_search();
    star_chain_tree.best_chains[1..limit + 1].iter().map(|x| x - 1).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(200).to_string()
}

problem!(answer, "1582");
