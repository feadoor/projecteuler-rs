//! [Problem 55 (Lychrel numbers)](https://projecteuler.net/problem=55)
//!
//! # Solution detail
//!
//! Simply check each number up to 10,000 to see if it is a Lychrel number, remembering to use
//! arbitrary-precision integers since the numbers involved can grow very large.

#[macro_use]
extern crate projecteuler_rs;
extern crate num;

use num::{BigUint, Num};

/// Calculates the result of reversing the digits of the given number.
fn reverse(x: &BigUint) -> BigUint {
    let x_string = x.to_str_radix(10);
    let reversed_digits: String = x_string.chars().rev().collect();
    BigUint::from_str_radix(&reversed_digits, 10).unwrap()
}

/// Determines whether the given number is a possible Lychrel number, as determined by performing
/// the reverse-and-add iteration up to the given threshold number of iterations.
fn is_lychrel(x: usize, threshold: usize) -> bool {
    let start = From::from(x);
    let mut worker = reverse(&start) + start;

    for _ in 0..threshold {
        let reversed_worker = reverse(&worker);
        if reversed_worker == worker {
            return false;
        }
        worker = worker + reversed_worker;
    }

    true
}

/// Find the number of Lychrel numbers below the given limit, as determined by performing the
/// reverse-and-add iteration up to the given threshold number of iterations.
fn solve(limit: usize, threshold: usize) -> usize {
    (1..limit).filter(|&x| is_lychrel(x, threshold)).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000, 50).to_string()
}

problem!(answer, "249");
