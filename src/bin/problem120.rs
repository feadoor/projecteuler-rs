//! [Problem 120 (Square remainders)](https://projecteuler.net/problem=120)
//!
//! # Solution detail
//!
//! This is a problem with a clever solution.
//!
//! First note that, carrying out a simple binomial expansion, the remainder of
//! `(a - 1)ⁿ + (a + 1)ⁿ` when divided by `a²` is:
//!
//! `(-1)ⁿ + (-1)ⁿ⁻¹ * na + 1 + na`
//!
//! When `n` is even, this is equal to `2`, and when `n` is odd, it is equal to `2na`. So the
//! maximum remainder, for fixed `a` and as `n` varies is simply the largest value attained by the
//! expression `2na (mod a²)` as `n` ranges over all odd integers.
//!
//! For odd `a`, this maximum is attained at `n = (a - 1) / 2` or `n = (3a - 1) / 2`, whichever
//! happens to be odd - giving a remainder of `a² - a`.
//!
//! For even `a`, the maximum comes at `n = (a - 2) / 2` or `n = (2a - 2) / 2`, whichever is odd,
//! this time giving a remainder of `a² - 2a`.
//!
//! The sum over all `a` can therefore be calculated by calculating the sum of `a² - a` for all `a`
//! in the range, and subtracting on top of that the sum of all even `a` in the range. This can
//! be done in constant time using the well-known formulae for the triangular and pyramidal numbers.

use projecteuler_rs::problem;

/// Find the sum of r_max over all 3 ≤ a ≤ lim.
fn solve(lim: usize) -> usize {
    let sum_of_squares = lim * (lim + 1) * (2 * lim + 1) / 6 - 5;
    let sum_of_a = lim * (lim + 1) / 2 - 3;
    let sum_of_even_a = ((lim / 2) * (lim / 2 + 1)) - 2;

    sum_of_squares - sum_of_a - sum_of_even_a
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1000).to_string()
}

problem!(answer, "333082500");
