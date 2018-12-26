//! [Problem 66 (Diophantine equation)](https://projecteuler.net/problem=66)
//!
//! # Solution detail
//!
//! The minimal solution to the Pell equation x<sup>2</sup> - Dy<sup>2</sup> = ±1 is given by the
//! convergent arising just before the first full period of the continued fraction expansion of √D.
//!
//! Letting this solution be p, q, if p<sup>2</sup> - Dq<sup>2</sup> = 1, then we have found the
//! solution.
//!
//! Otherwise, if p<sup>2</sup> - Dq<sup>2</sup> = -1, then the smallest solution to the equation
//! x<sup>2</sup> - Dy<sup>2</sup> = 1 is given by:
//!
//! x = p<sup>2</sup> + Dq<sup>2</sup>, y = 2pq
//!
//! We can use the above process to generate the minimal solution for each D, and then simply keep
//! track of the largest value of x that we find.

use continued_fractions::{ContinuedFractionConvergents, PeriodicContinuedFraction};
use num::{BigUint, Zero, One};
use number_theory::is_square;
use projecteuler_rs::problem;

/// Finds the minimal solution (x, y) to the Pell equation with the given D.
fn minimal_solution(d: u64) -> (BigUint, BigUint) {
    let expansion = PeriodicContinuedFraction::sqrt(d);
    let (mut terms, period) = (expansion.tail, expansion.period);
    terms.extend_from_slice(&period[..period.len() - 1]);

    let mut last_convergent = (BigUint::zero(), BigUint::zero());
    for convergent in ContinuedFractionConvergents::new(terms.into_iter()) {
        last_convergent = convergent;
    }

    let (p, q) = last_convergent;
    if &p * &p == d * &q * &q + BigUint::one() {
        (p, q)
    } else {
        (&p * &p + d * &q * &q, 2u32 * p * q)
    }
}

/// Find the value of D, not exceeding the given limit, for which the Pell equation in D D
/// has the largest value of x in its minimal solution.
fn solve(n: u64) -> u64 {
    (1..n + 1)
        .filter(|&d| !is_square(d))
        .max_by_key(|&d| minimal_solution(d).0)
        .unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "661");
