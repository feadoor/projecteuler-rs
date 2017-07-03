//! [Problem 20 (Factorial digit sum)](https://projecteuler.net/problem=20)
//!
//! # Problem statement
//!
//! n! means n × (n − 1) × ... × 3 × 2 × 1
//!
//! For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
//! and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//!
//! Find the sum of the digits in the number 100!
//!
//! # Solution detail
//!
//! The only thing worth noting here is that the result of the factorial will never fit into a
//! primitive type, so we should use `BigUint`. The rest is just a matter of doing the
//! calculation and summing the digits.

use num::BigUint;

/// The name of the problem.
pub const NAME: &'static str = "Problem 20";
/// A description of the problem.
pub const DESC: &'static str = "Factorial digit sum";

/// Calculate n! (factorial).
fn factorial(n: u64) -> BigUint {
    let mut result = From::from(1u64);
    for i in 2..n + 1 {
        result = result * <BigUint as From<u64>>::from(i);
    }
    result
}

/// Find the sum of the digits of n! (factorial).
fn solve(n: u64) -> u64 {
    let result = factorial(n);
    result.to_str_radix(10).as_bytes().iter().map(|x| (x - b'0') as u64).sum()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(100).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem020() {
        assert_eq!(super::answer(), "648");
    }
}
