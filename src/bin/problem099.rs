//! [Problem 99 (Largest exponential](https://projecteuler.net/problem=99)
//!
//! # Solution detail
//!
//! In order to avoid calculating with very large integers, we can use the facts that:
//!
//!   - `log` is an increasing function
//!   - `log(aᵇ) = b log(a)`
//!
//! This means that `aᵇ > cᵈ` if, and only if, `b log(a) > d log(c)`, and we can use this to
//! calculate the largest number in the file.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use projecteuler_rs::problem;

/// Find the index (1-indexed) of the tuple `(a, b)` for which `aᵇ` is largest.
fn solve(pairs: &[(f64, f64)]) -> usize {
    let (mut max_val, mut max_idx) = (0.0, 0);
    for (idx, (a, b)) in pairs.iter().enumerate() {
        if b * a.ln() > max_val {
            max_val = b * a.ln(); max_idx = idx;
        }
    }
    max_idx + 1
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem099.txt")).unwrap();
    let reader = BufReader::new(file);
    let pairs: Vec<_> = reader.lines()
        .map(|line| line.unwrap().split(',').map(|x| x.to_string()).collect::<Vec<_>>())
        .map(|parts| (parts[0].parse().unwrap(), parts[1].parse().unwrap()))
        .collect();
    solve(&pairs).to_string()
}

problem!(answer, "709");
