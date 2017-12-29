//! [Problem 56 (Powerful digit sum)](https://projecteuler.net/problem=56)
//!
//! # Solution detail
//!
//! The only thing worth noting here is that the result of the exponentiation will never fit into
//! a primitive type, so we should use `BigUint`. The rest is just a matter of doing the
//! calculations and summing the digits, keeping track of the best digital sum as we go.

#[macro_use]
extern crate projecteuler_rs;
extern crate num;

use num::BigUint;
use num::pow::pow;

/// Calculate the sum of the digits (in base 10) of the given number.
fn sum_of_digits(x: BigUint) -> u64 {
    x.to_str_radix(10).as_bytes().iter().map(|x| (x - b'0') as u64).sum()
}

/// Find the largest digital sum among numbers of the form a^b, with a, b less than the given limit.
fn solve(limit: usize) -> u64 {
    let mut best_sum = 0;
    for a in 1..limit {
        for b in 1..limit {
            let digital_sum = sum_of_digits(pow(From::from(a), b));
            if digital_sum > best_sum {
                best_sum = digital_sum;
            }
        }
    }

    best_sum
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "972");
