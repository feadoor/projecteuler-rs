//! [Problem 126 (Cuboid layers)](https://projecteuler.net/problem=126)
//!
//! # Solution detail
//!
//! It's slightly fiddly to calculate the number of cubes in each layer, but it can be done! If the
//! side lengths of the cuboid are `a`, `b` and `c` then the number of cubes in a single layer is:
//!
//! `2(ab + bc + ca) + 4(n-1)(a + b + c - 1) + 4(n-1)²`
//!
//! Then, brute force is our best friend. Assuming that the answer is below a particular limit,
//! then just iterate over all cuboids, and over all layers, storing the number of times each
//! number of cubes has been seen, for each number of cubes up to the limit. If any number of cubes
//! has been used the right number of times, then that's the answer. Otherwise, increase the limit
//! a little (e.g. double it) and start again.

use std::iter::successors;
use projecteuler_rs::problem;

/// Calculate the number of times each layer size is used, up to a limit.
fn layer_size_counts(limit: usize) -> Vec<usize> {

    // Store the number of times each layer size is seen.
    let mut counts = vec![0; limit];

    // Iterate over all possible cuboids with sides a < b < c
    for a in 1..(limit - 1) / 2 {
        for b in (1..(a + 1)).take_while(|&b| 2 * (a * b + a + b) < limit) {
            for c in (1..(b + 1)).take_while(|&c| 2 * (a * b + b * c + c * a) < limit) {

                // Iterate over the number of cubes in each layer of this cuboid
                let size = |n| 2 * (a * b + b * c + c * a) + 4 * (n - 1) * (a + b + c + n - 2);
                for layer_size in (1..).map(size).take_while(|&x| x < limit) {
                    counts[layer_size] += 1;
                }
            }
        }
    }

    counts
}

/// Find the first layer size which occurs exactly `n` times, among all cuboids.
fn smallest_size_with_n_occurrences(n: usize) -> usize {
    for limit in successors(Some(1000), |x| Some(2 * x)) {
        let counts = layer_size_counts(limit);
        if let Some(idx) = counts.iter().position(|&x| x == n) {
            return idx;
        }
    }

    unreachable!();
}

/// Find the smallest layer size which appears exactly the given number of times.
fn solve(target: usize) -> usize {
    smallest_size_with_n_occurrences(target)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1000).to_string()
}

problem!(answer, "18522");
