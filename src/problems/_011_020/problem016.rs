//! [Problem 16 (Power digit sum)](https://projecteuler.net/problem=16)
//!
//! # Problem statement
//!
//! 2<sup>15</sup> = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//!
//! What is the sum of the digits of the number 2<sup>1000</sup>?
//!
//! # Solution detail
//!
//! The only thing worth noting here is that the result of the exponentiation will never fit into
//! a primitive type, so we should use `BigUint`. The rest is just a matter of doing the
//! calculation and summing the digits.

#[macro_use]
extern crate projecteuler_rs;
extern crate num;

use num::BigUint;
use num::pow::pow;

/// Find the sum of the digits of base ^ exp.
fn solve(base: u64, exp: usize) -> u64 {
    let result: BigUint = pow(From::from(base), exp);
    result.to_str_radix(10).as_bytes().iter().map(|x| (x - b'0') as u64).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2, 1_000).to_string()
}

problem!(answer, "1366");
