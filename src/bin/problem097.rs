//! [Problem 97 (Large non-Mersenne prime)](https://projecteuler.net/problem=97)
//!
//! # Solution detail
//!
//! Use an efficient modular exponentiation algorithm to calculate 2<sup>7830457</sup> modulo
//! 10<sup>10</sup>, then multiply by 28433 and add one, still working modulo 10<sup>10</sup>.

use modular_arithmetic::mod_exp;
use projecteuler_rs::problem;

/// Calculate the last 10 digits of the large non-Mersenne prime
fn solve() -> u64 {
    let modulus = 10_000_000_000;
    (28433 * mod_exp(2, 7_830_457, modulus) + 1) % modulus
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "8739992577");
