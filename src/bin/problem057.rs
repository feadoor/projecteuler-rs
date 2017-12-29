//! [Problem 57 (Square root convergents)](https://projecteuler.net/problem=57)
//!
//! # Solution detail
//!
//! There is nothing more clever to do here than to just iteratively generate the first one thousand
//! expansions, and for each one, check if the numerator has more digits than the denominator.
//!
//! Note that the problem statement does not consider the first expansion, 1/1, to be valid, so we
//! should skip it.

#[macro_use]
extern crate projecteuler_rs;
extern crate continued_fractions;

use continued_fractions::PeriodicContinuedFraction;

/// Find how many of the first `num_expansions` expansions of the square root of `n` have more
/// digits in the numerator than the denominator.
fn solve(n: u64, num_expansions: usize) -> usize {
    PeriodicContinuedFraction::sqrt(n).convergents()
        .skip(1)
        .take(num_expansions)
        .filter(|&(ref numer, ref denom)| numer.to_string().len() > denom.to_string().len())
        .count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2, 1000).to_string()
}

problem!(answer, "153");
