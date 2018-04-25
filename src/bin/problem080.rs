//! [Problem 80 (Square root digital expansion)](https://projecteuler.net/problem=80)
//!
//! # Solution detail
//!
//! Good approximations to square roots can be found using the convergents to the continued fraction
//! expansion. In particular, if `p / q` is a convergent to an irrational number `x`, then the
//! absolute error in this approximation is at most `1 / q^2`.
//!
//! This means that we just need to find a convergent to √n for which the denominator has half as
//! many digits as the required level of accuracy, and then calculate the first decimal digits of
//! this approximation.

#[macro_use]
extern crate projecteuler_rs;
extern crate continued_fractions;
extern crate number_theory;
extern crate num;

use continued_fractions::PeriodicContinuedFraction;
use number_theory::is_square;
use num::BigUint;
use num::pow::pow;

/// Finds 10 to the power of `n`
fn power_of_ten(n: usize) -> BigUint {
    pow(From::from(10usize), n)
}

/// Find the sum of the first digits of the square root of the given number.
fn first_digits_sqrt_sum(n: u64, digits: usize) -> usize {
    PeriodicContinuedFraction::sqrt(n).convergents()
        .find(|&(_, ref denom)| denom.to_str_radix(10).len() > digits / 2 + 1)
        .map(|(numer, denom)| power_of_ten(digits) * numer / denom)
        .map(|x: BigUint| x.to_str_radix(10).bytes().take(100).map(|b| (b - b'0') as usize).sum())
        .unwrap()
}

/// Find the sum of the first decimal digits of approximations to square roots of the first integers
fn solve(limit: u64, digits: usize) -> usize {
    (1..limit + 1)
        .filter(|&x| !is_square(x))
        .map(|x| first_digits_sqrt_sum(x, digits))
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100, 100).to_string()
}

problem!(answer, "40886");
