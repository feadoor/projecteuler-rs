//! [Problem 72 (Counting fractions)](https://projecteuler.net/problem=72)
//!
//! # Problem statement
//!
//! Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
//! called a reduced proper fraction.
//!
//! If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//!
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
//! 6/7, 7/8
//!
//! It can be seen that there are 21 elements in this set.
//!
//! How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?
//!
//! # Solution detail
//!
//! Let g(n) denote the number of pairs (x, y) with 1 <= x <= y <= n and gcd(x, y) = 1. We are
//! looking for the value of g(1,000,000).

//! By multiplying pairs by a common factor k, it follows that g(n / k) is equal to the number of
//! pairs (x, y) with x, y <= n and gcd(x, y) = k. Summing over all k, we find that:
//!
//! sum[k = 1 to n] g(n / k) = n(n + 1) / 2
//!
//! This is because both sides are equal to the total number of pairs x, y with 1 <= x <= y <= n.
//!
//! Isolated values of functions defined by this style of recurrence relation can be computed
//! quickly using the `mertens_recurrence` crate.

#[macro_use]
extern crate projecteuler_rs;
extern crate mertens_recurrence;

use mertens_recurrence::Recurrence;

/// Find the number of reduced fractions with denominators up to the given limit.
fn solve(limit: u64) -> i64 {
    let recurrence = Recurrence::new(|n| (n * (n + 1) / 2) as i64, |b, c| (c - b) as i64);
    recurrence.calculate_value_at(limit) - 1 // Subtract one since 1/1 is not a proper fraction
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "303963552391");
