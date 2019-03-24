//! [Problem 88 (Product-sum numbers)](https://projecteuler.net/problem=88)
//!
//! # Solution detail
//!
//! For a given `k`, the sequence (k, 2, 1, 1, ..., 1) is a sum-product sequence, so the minimal
//! sum-product number is at most `2k`.
//!
//! We can therefore use the following algorithm to find the minimal sum-product numbers below
//! a specified limit:
//!
//!  - Generate all non-decreasing sequences of integers ≥ 2 with product at most 2 × lim
//!  - Pad each sequence with 1s until it becomes a sum-product sequence
//!  - Check if the new sequence is a new minimal product-sum sequence for that number of terms
//!
//! Since the second and third of these steps only depends on the sum, product and length of the
//! sequence, we don't actually need to generate the sequences themselves - just the sums, products
//! and lengths will suffice.

use itertools::Itertools;
use projecteuler_rs::problem;
use search::{DepthFirstTree, Pruning};

/// Important data about a sequence of numbers
struct SequenceData {
    /// The sum of the sequence
    sum: usize,
    /// The product of the sequence
    product: usize,
    /// The length of the sequence
    length: usize,
}

/// The information held about the current sequence during a depth-first search of all sequences
struct SequenceTree {
    /// The sum of the current sequence
    sum: usize,
    /// The product of the current sequence
    product: usize,
    /// The values in the current sequence
    values: Vec<usize>,
    /// The maximum product of sequences to find
    maximum_product: usize,
}

/// A description of a step that can be taken in the search tree.
struct SequenceTreeStep {
    /// The next number to add to the sequence
    next_value: usize,
}

impl SequenceTree {

    /// Construct a new `SequenceTree`
    fn new(maximum_product: usize) -> SequenceTree {
        SequenceTree {
            sum: 0,
            product: 1,
            values: Vec::new(),
            maximum_product: maximum_product,
        }
    }

    fn last_value(&self) -> usize {
        self.values.last().map(|x| *x).unwrap_or(2)
    }
}

impl DepthFirstTree for SequenceTree {
    type Step = SequenceTreeStep;
    type Output = SequenceData;

    /// Return all possible choices for the next value to add to the current sequence.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        (self.last_value()..self.maximum_product / self.product + 1).map(|next_value| Self::Step {
            next_value: next_value
        }).collect()
    }

    /// Never prune this tree
    fn should_prune(&mut self) -> Pruning {
        Pruning::None
    }

    /// Add the next digit to the sequence
    fn apply_step(&mut self, step: &Self::Step) {
        self.sum += step.next_value;
        self.product *= step.next_value;
        self.values.push(step.next_value);
    }

    /// Remove the last digit from the sequence
    fn revert_step(&mut self, step: &Self::Step) {
        self.sum -= step.next_value;
        self.product /= step.next_value;
        self.values.pop();
    }

    /// Output the key stats of the current sequence
    fn output(&mut self) -> Option<Self::Output> {
        Some(SequenceData { sum: self.sum, product: self.product, length: self.values.len() })
    }
}

/// Find the sum of the minimal product-sum numbers for 2 ≤ k ≤ `limit`
fn solve(limit: usize) -> usize {

    let mut minimal_products: Vec<_> = (0..limit + 1).map(|k| 2 * k).collect();

    for data in SequenceTree::new(2 * limit).into_iter() {
        let ones_to_add = data.product - data.sum;
        let k = data.length + ones_to_add;
        if k <= limit && minimal_products[k] > data.product {
            minimal_products[k] = data.product;
        }
    }

    minimal_products[2..].iter().unique().sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(12_000).to_string()
}

problem!(answer, "7587457");
