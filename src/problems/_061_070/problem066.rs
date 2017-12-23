//! [Problem 66 (Diophantine equation)](https://projecteuler.net/problem=66)
//!
//! # Problem statement
//!
//! Consider quadratic Diophantine equations of the form:
//!
//! x<sup>2</sup> – Dy<sup>2</sup> = 1
//!
//! For example, when D=13, the minimal solution in x is 649<sup>2</sup> – 13 × 180<sup>2</sup> = 1.
//!
//! It can be assumed that there are no solutions in positive integers when D is square.
//!
//! By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:
//!
//! 3<sup>2</sup> – 2×2<sup>2</sup> = 1
//! 2<sup>2</sup> – 3×1<sup>2</sup> = 1
//! 9<sup>2</sup> – 5×4<sup>2</sup> = 1
//! 5<sup>2</sup> – 6×2<sup>2</sup> = 1
//! 8<sup>2</sup> – 7×3<sup>2</sup> = 1
//!
//! Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.
//!
//! Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is
//! obtained.
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

#[macro_use]
extern crate projecteuler_rs;
extern crate continued_fractions;
extern crate num;
extern crate number_theory;

use continued_fractions::{ContinuedFractionConvergents, PeriodicContinuedFraction};
use num::{BigUint, Zero, One};
use number_theory::is_square;

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
