//! [Problem 106 (Special subset sums: meta-testing)](https://projecteuler.net/problem=106)
//!
//! # Solution detail
//!
//! We will first consider the question: given a set of size n, containing elements
//! `a₁ < a₂ < ... < aₙ`, and given `s` with `2s ≤ n`, how many pairs of subsets of size `s` are
//! there for which we can guarantee that one will be smaller than the other?
//!
//! For example, with `s = 2`, we know for sure that `a₁ + a₃ < a₂ + a₄` because the elements on
//! each side can be paired up in such a way that everything on the left is less than its
//! corresponding element on the right.
//!
//! This question can be answered easily by first imagining the choice of `2s` elements to be fixed,
//! and asking how many ways there are of splitting them up such that this pairing is possible.
//!
//! The assignment of each element `a₁, a₂, ...` in turn to either the left or right hand side can
//! be viewed as a journey from the top-left corner to the bottom-right corner on a square grid of
//! size `s`. Taking an element to the left is a step down, and an element to the right is a step
//! right.
//!
//! The pairing is possible if we never cross the diagonal - that is, at no point are there more
//! elements on the right than on the left. The number of ways of doing this is well-known to be the
//! Catalan number Cₛ.
//!
//! It follows that the number of pairs of subsets of size `s` which do _not_ need to be tested is
//! given by the expression `binom(n, 2s) * Cₛ`.
//!
//! We can therefore calculate how many pairs _do_ need to be tested by summing this expression for
//! each value of `s`, and subtracting from the total number of subset pairs.

use number_theory::binom;
use projecteuler_rs::problem;

/// Find the Catalan number `Cₛ`
fn catalan(s: usize) -> usize {
    binom(2 * s, s) / (s + 1)
}

/// Find the number of pairs of subsets of size `s`, from a set of size `n`, that do _not_ need to
/// be tested for equality.
fn trivial_pairs(n: usize, s: usize) -> usize {
    binom(n, 2 * s) * catalan(s)
}

/// Find the total number of pairs of subsets of size `s` from a set of size `n`
fn total_pairs(n: usize, s: usize) -> usize {
    binom(n, s) * binom(n - s, s) / 2
}

/// Find the number of pairs of subsets that must be tested when checking for a special sum set of
/// size `n`.
fn solve(n: usize) -> usize {
    (1..n / 2 + 1).map(|s| total_pairs(n, s) - trivial_pairs(n, s)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(12).to_string()
}

problem!(answer, "21384");
