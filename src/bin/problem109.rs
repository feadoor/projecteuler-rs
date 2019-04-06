//! [Problem 109 (Darts)](https://projecteuler.net/problem=109)
//!
//! # Solution detail
//!
//! This is an easy application of brute force - simply iterate over all combinations of two darts
//! (including a miss) plus a double, and count how many have a total less than 100.
//!
//! Be careful when iterating over the choice for the first two darts that the same two darts in
//! different orders are not counted twice.

use projecteuler_rs::problem;
use std::iter;

/// Find the number of darts checkouts for scores less than the given limit.
fn solve(limit: usize) -> usize {

    // Write down all the scores that are possible with a single dart
    let singles = (0..21).chain(iter::once(25));
    let doubles = (1..21).chain(iter::once(25)).map(|x| 2 * x);
    let trebles = (1..21).map(|x| 3 * x);

    let scores_vec: Vec<usize> = singles.chain(doubles).chain(trebles).collect();
    let doubles_vec: Vec<usize> = (1..21).chain(iter::once(25)).map(|x| 2 * x).collect();

    // Iterate over combinations and count the ones with a small enough total
    let mut result = 0;
    for ix in 0..scores_vec.len() {
        for jx in ix..scores_vec.len() {
            let base_score = scores_vec[ix] + scores_vec[jx];
            result += doubles_vec.iter().take_while(|&d| base_score + d < limit).count();
        }
    }
    result
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "38182");
