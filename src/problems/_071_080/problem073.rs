//! [Problem 73 (Counting fractions in a range)](https://projecteuler.net/problem=73)
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
//! It can be seen that there are 3 fractions between 1/3 and 1/2.
//!
//! How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for
//! d ≤ 12,000?
//!
//! # Solution detail
//!
//! Let g(n) denote the number of pairs (x, y) with 1 <= x <= y < 3x < 6y <= 6n and gcd(x, y) = 1.
//! We are looking for the value of g(12,000).

//! By multiplying pairs by a common factor k, it follows that g(n/k) is equal to the number of
//! pairs (x, y) with 1 <= x <= y < 3x < 6y <= 6n and gcd(x, y) = k. Summing over all k, we find
//! that:
//!
//! sum[k = 1 to n] g(n / k) = f(n)
//!
//! where f(n) is equal to the total number of pairs 1 <= x <= y < 3x < 6y <= 6n, with no condition
//! on the gcd of x and y.
//!
//! This function f(n) can be calculated in constant time by splitting into cases depending on the
//! value of n modulo 6. Firstly, note that by summing over the possible values of y, we have:
//!
//! `f(N) = sum[m = 1 to N] floor((m - 1) / 2) - floor(m / 3)`
//!
//! If we let N = 6q + r, so that the sum up to 6q ends on a multiple of 6, with 0 <= r < 6, this
//! boils down to:
//!
//! ```text
//! f(N) = f(6q) + (remainder terms)
//!      = 2 × 3q(3q - 1) / 2 - 3 × 2q(2q - 1) / 2 - 2q + (remainder terms)
//!      = 3q(3q - 1 - 2q + 1) - 2q + (remainder terms)
//!      = q(3q - 2) + (remainder terms)
//! ```
//!
//! The remainder terms depend on r as follows:
//!
//!   - r = 0: no remainder
//!   - r = 1: q
//!   - r = 2: 2q
//!   - r = 3: 3q
//!   - r = 4: 4q
//!   - r = 5: 5q + 1
//!
//! Thus the final value of f(N) is q(3q - 2 + r) (+1 if r = 5)
//!
//! Isolated values of functions defined by this style of recurrence relation can be computed
//! quickly using the `mertens_recurrence` crate.

#[macro_use]
extern crate projecteuler_rs;
extern crate mertens_recurrence;

use mertens_recurrence::Recurrence;

/// Find the number of reduced fractions in the range (1/3, 1/2) with denominators up to the given
/// limit.
fn solve(limit: u64) -> i64 {
    let f = |n| {
        let (q, r) = (n / 6, n % 6);
        (q * (3 * q - 2 + r) + if r == 5 { 1 } else { 0 }) as i64
    };

    let recurrence = Recurrence::new(f, |b, c| (c - b) as i64);
    recurrence.calculate_value_at(limit)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(12_000).to_string()
}

problem!(answer, "7295372");
