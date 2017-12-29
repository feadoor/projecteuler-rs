//! [Problem 22 (Names scores)](https://projecteuler.net/problem=22)
//!
//! # Solution detail
//!
//! There's no clever trick going on here. Just read in the names from file, sort them, calculate
//! each name score and find the sum.

#[macro_use]
extern crate projecteuler_rs;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

/// Find the sum of the scores of the given list of names.
fn solve(names: &mut [String]) -> u64 {
    // First, sort the given names.
    names.sort();

    // Now iterate over the names and calculate the name scores.
    let score = |name: &str| -> u64 { name.as_bytes().iter().map(|x| (x - b'A' + 1) as u64).sum() };
    names.iter().enumerate().map(|(ix, name)| (ix + 1) as u64 * score(name)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem022.txt")).unwrap();
    let reader = BufReader::new(file);
    let mut names: Vec<String> = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim_matches('"').to_string())
        .collect();
    solve(&mut names).to_string()
}

problem!(answer, "871198282");
