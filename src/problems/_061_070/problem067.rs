//! [Problem 67 (Maximum path sum II)](https://projecteuler.net/problem=67)
//!
//! # Solution detail
//!
//! This is a classic example of a problem which can be solved with dynamic programming. For each
//! cell in the triangle, we will calculate the maximum value of a path ending at that cell. The
//! solution is then the largest of these numbers.
//!
//! To calculate these numbers, start at the top of the triangle and work down. For the apex, the
//! maximum path sum is simply the value in that cell. For every other cell, the maximum path sum
//! is the number in that cell, plus the greater of the two maximums from the cells directly above
//! it.

#[macro_use]
extern crate projecteuler_rs;

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Find the maximum sum of a path through the given triangular grid, given as a vector of vectors
/// and guaranteed to be of valid dimensions.
fn solve(grid: &[Vec<u64>]) -> u64 {

    // Keep track of the maximum path sums from the previous row.
    let mut prev_row = vec![0; grid.len()];
    prev_row[0] = grid[0][0];

    // Iterate over the rows of the triangle and find the maximum sums for the next row.
    for ix in 1..grid.len() {
        let mut curr_row = vec![0; grid.len()];
        for jx in 0..ix + 1 {
            let left = if jx > 0 { prev_row[jx - 1] } else { 0 };
            let right = if jx < ix { prev_row[jx] } else { 0 };
            curr_row[jx] = grid[ix][jx] + max(left, right);
        }
        prev_row = curr_row;
    }

    *prev_row.iter().max().unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let parse_row = |x: &str| -> Vec<u64> {
        x.split_whitespace().map(|x| u64::from_str_radix(x, 10).unwrap()).collect()
    };

    let file = File::open(&Path::new("inputs/problem067.txt")).unwrap();
    let reader = BufReader::new(file);
    let grid: Vec<Vec<u64>> = reader.lines()
        .map(|line| parse_row(&line.unwrap()))
        .collect();

    solve(&grid).to_string()
}

problem!(answer, "7273");
