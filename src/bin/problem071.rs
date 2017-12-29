//! [Problem 71 (Ordered fractions)](https://projecteuler.net/problem=71)
//!
//! # Solution detail
//!
//! For each possible denominator d, the fraction immediately to the left of 3/7 is:
//!
//! ((3 * d - 1) / 7) / d
//!
//! Simply iterate over all choices of d, calculate the above fraction, reduce it to its lowest
//! terms and keep track of the best fraction we have found so far.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use number_theory::gcd;

/// Find the numerator of the largest fraction with denominator not exceeding the given limit which
/// is less than the given fraction a / b.
fn solve(limit: u64, a: u64, b: u64) -> u64 {
    let mut best_numerator = 0;
    let mut best_denominator = 1;

    for denominator in 2..limit + 1 {
        let numerator = (a * denominator - 1) / b;
        if numerator * best_denominator > best_numerator * denominator {
            best_numerator = numerator;
            best_denominator = denominator;
        }
    }

    best_numerator / gcd(best_numerator, best_denominator)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000, 3, 7).to_string()
}

problem!(answer, "428570");
