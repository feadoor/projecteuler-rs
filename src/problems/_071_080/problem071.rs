//! [Problem 71 (Ordered fractions)](https://projecteuler.net/problem=71)
//!
//! # Problem statement
//!
//! Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is called a reduced proper fraction.
//!
//! If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//!
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
//! 6/7, 7/8
//!
//! It can be seen that 2/5 is the fraction immediately to the left of 3/7.
//!
//! By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size,
//! find the numerator of the fraction immediately to the left of 3/7.
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
