//! [Problem 65 (Convergents of e)](https://projecteuler.net/problem=65)
//!
//! # Solution detail
//!
//! Simply calculate the 100th convergent and sum its digits - nothing more clever than that!

use std::iter::repeat;

use continued_fractions::ContinuedFractionConvergents;
use interleave::*;
use projecteuler_rs::problem;

/// Find the sum of the digits of the numerator of the given convergent of the continued fraction
/// expansion of e.
fn solve(n: usize) -> u64 {
    let e_continued_fraction = Some(2).into_iter().chain(
        interleave!(repeat(1), (2..).step_by(2), repeat(1))
    );

    ContinuedFractionConvergents::new(e_continued_fraction)
        .skip(n - 1)
        .next().unwrap().0
        .to_str_radix(10).as_bytes().iter()
        .map(|x| (x - b'0') as u64)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "272");
