//! [Problem 57 (Square root convergents)](https://projecteuler.net/problem=57)
//!
//! # Problem statement
//!
//! It is possible to show that the square root of two can be expressed as an infinite continued
//! fraction.
//!
//! √2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
//!
//! By expanding this for the first four iterations, we get:
//!
//! - 1 + 1/2 = 3/2 = 1.5
//! - 1 + 1/(2 + 1/2) = 7/5 = 1.4
//! - 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
//! - 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
//!
//! The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985,
//! is the first example where the number of digits in the numerator exceeds the number of digits in
//! the denominator.
//!
//! In the first one-thousand expansions, how many fractions contain a numerator with more digits
//! than denominator?
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
