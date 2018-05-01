//! [Problem 86 (Cuboid route)](https://projecteuler.net/problem=86)
//!
//! # Solution detail
//!
//! Supposing that a given cuboid has side lengths x ≤ y≤ z, then the shortest path between
//! opposite corners is equal to sqrt((x + y)^2 + z^2), by "unfolding" the cuboid along its longest
//! sides.
//!
//! We can use the following process to find the number of cuboids with integer shortest path and
//! maximum side-length equal to m:
//!
//! 1. Iterate over each k = m + 1, m + 2, m + 3...,
//!
//! 2. Let d = k^2 - m^2. If D > 4m^2 then stop processing. Otherwise, let s = sqrt(d) and
//!    and count how many 1 ≤ x ≤ y ≤ m there are with x + y = s. Add this count the the total
//!    number of solutions found so far.
//!
//! If we finish processing a particular m, and the solution count is now large enough, then we have
//! found our answer.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;
extern crate itertools;

use number_theory::{is_square, integer_sqrt};
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

/// Find the number of pairs x, y with 1 ≤ x ≤ y ≤ lim and x + y = d
fn pairs_with_sum(d: u64, lim: u64) -> u64 {
    if d <= lim {
        d / 2
    } else if d <= 2 * lim {
        (2 * (lim + 1) - d) / 2
    } else {
        0
    }
}

/// Find the number of cuboids with an integer shortest path and longest side equal to m.
fn cuboids_with_longest_side(m: u64) -> u64 {
    let d_limit = 4 * m * m;
    (m + 1..).map(|k| k * k - m * m)
        .take_while(|&d| d <= d_limit)
        .filter(|&d| is_square(d))
        .map(|d| pairs_with_sum(integer_sqrt(d), m))
        .sum()
}

/// Find the least value of m such that there are at least the given number of cuboids, with longest
/// side not exceeding m, with an integer shortest path.
fn solve(target: u64) -> u64 {
    (1..).map(|m| cuboids_with_longest_side(m as u64))
        .enumerate()
        .fold_while(0, |acc, (idx, val)| {
            if acc >= target { Done(idx as u64) } else { Continue(acc + val) }
        }).into_inner()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "1818");
