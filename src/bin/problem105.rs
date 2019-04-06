//! [Problem 105 (Special subset sums: testing)](https://projecteuler.net/problem=105)
//!
//! # Solution detail
//!
//! Given that there are only 100 sets to test, and the largest set only has 12 elements, brute
//! force is the way to go here! A set of size 12 only has 4096 subsets, and it's easy enough to
//! check them all for distinct sums.
//!
//! The second condition is even easier - if the elements of the set are a₁, a₂, ..., aₙ sorted in
//! increasing order, then to check that S(B) > S(C) whenever B is larger than C we just need to
//! check:
//!
//! `aₙ < a₁ + a₂`
//!
//! `aₙ + aₙ₋₁ < a₁ + a₂ + a₃
//!
//! etc.
//!
//! So we'll simply iterate over all the sets in the file, check each one to see if it's special,
//! and count the ones that are.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::collections::HashSet;

use projecteuler_rs::problem;

/// Check if the given set has distinct subset sums - simply by generating them all iteratively
/// and checking that there are the right number of them.
fn distinct_subset_sums(set: &[usize]) -> bool {
    let mut sums = HashSet::new(); sums.insert(0);
    for (ix, elem) in set.iter().enumerate() {
        let next_sums = sums.iter().map(|s| s + elem).collect();
        sums = sums.union(&next_sums).map(|x| *x).collect();
        if sums.len() != (1 << (ix + 1)) { return false; }
    }
    true
}

/// Check if the given set satisfies the condition that larger subsets must have larger sums.
fn large_subset_sums(set: &[usize]) -> bool {
    let mut sorted_set = set.to_vec(); sorted_set.sort();
    for length in 2..sorted_set.len() {
        let first_sum: usize = sorted_set[..length].iter().sum();
        let second_sum: usize = sorted_set[sorted_set.len() - length + 1..].iter().sum();
        if first_sum <= second_sum { return false; }
    }
    true
}

/// Find the sum of the elements of the given sets which are special sum sets.
fn solve(sets: &[Vec<usize>]) -> usize {
    sets.iter()
        .filter(|&set| large_subset_sums(set) && distinct_subset_sums(set))
        .map(|set| set.iter().sum::<usize>())
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem105.txt")).unwrap();
    let reader = BufReader::new(file);
    let sets: Vec<_> = reader.lines()
        .map(|line| line.unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .collect();
    solve(&sets).to_string()
}

problem!(answer, "73702");
