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

#![feature(generators, generator_trait)]

use generators::{GeneratorIteratorAdapter, yield_from};
use itertools::Itertools;
use projecteuler_rs::problem;
use std::ops::{Generator, GeneratorState};

/// Important data about a sequence of numbers
struct SequenceData {
    sum: usize,
    product: usize,
    length: usize,
}

/// Generate all non-decreasing sequences of numbers whose product is at most the given limit.
fn increasing_sequences<'a>(product_limit: usize) -> impl Iterator <Item=SequenceData> {

    #[derive(Clone, Copy)]
    struct SearchEnv   { product_limit: usize, }
    struct SearchState { last_value: usize, sum: usize, product: usize, length: usize, }

    /// The state that arises from the current state by appending a single term to the sequence
    fn next_state(state: &SearchState, term: usize) -> SearchState {
        SearchState {
            last_value: term,
            sum: state.sum + term,
            product: state.product * term,
            length: state.length + 1,
        }
    }

    // An inner function that facilitates the search by recursively calling itself with one more
    // term added to the sequence each time.
    fn nodes_beneath(state: SearchState, env: SearchEnv) -> Box<dyn Generator<Yield=SequenceData, Return=()>> {
        Box::new(move || {
            yield SequenceData { product: state.product, sum: state.sum, length: state.length };
            for next_value in state.last_value.. {
                let next_product = state.product * next_value;
                if next_product <= env.product_limit {
                    yield_from!(nodes_beneath(next_state(&state, next_value), env));
                } else { break; }
            }
        })
    }

    let initial_state = SearchState { last_value: 2, sum: 0, product: 1, length: 0 };
    let env = SearchEnv { product_limit: product_limit };
    GeneratorIteratorAdapter::of(nodes_beneath(initial_state, env))
}

/// Find the sum of the minimal product-sum numbers for 2 ≤ k ≤ `limit`
fn solve(limit: usize) -> usize {

    let mut minimal_products: Vec<_> = (0..limit + 1).map(|k| 2 * k).collect();

    for data in increasing_sequences(2 * limit) {
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
