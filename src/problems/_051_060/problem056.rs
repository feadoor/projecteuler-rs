//! [Problem 56 (Powerful digit sum)](https://projecteuler.net/problem=56)
//!
//! # Problem statement
//!
//! A googol (10<sup>100</sup>) is a massive number: one followed by one-hundred zeros;
//! 100<sup>100</sup> is almost unimaginably large: one followed by two-hundred zeros.
//! Despite their size, the sum of the digits in each number is only 1.
//!
//! Considering natural numbers of the form, a<sup>b</sup>, where a, b < 100, what is the maximum
//! digital sum?
//!
//! # Solution detail
//!
//! The only thing worth noting here is that the result of the exponentiation will never fit into
//! a primitive type, so we should use `BigUint`. The rest is just a matter of doing the
//! calculations and summing the digits, keeping track of the best digital sum as we go.

use num::BigUint;
use num::pow::pow;

/// The name of the problem.
pub const NAME: &'static str = "Problem 56";
/// A description of the problem.
pub const DESC: &'static str = "Powerful digit sum";

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
pub fn answer() -> String {
    solve(100).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem056() {
        assert_eq!(super::answer(), "972");
    }
}
