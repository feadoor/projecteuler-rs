//! [Problem 24 (Lexicographic permutations)](https://projecteuler.net/problem=24)
//!
//! # Problem statement
//!
//! A permutation is an ordered arrangement of objects. For example, 3124 is one possible
//! permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or
//! alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2
//! are:
//!
//! 012   021   102   120   201   210
//!
//! What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
//!
//! # Solution detail
//!
//! We can find the `n`th lexicographic permutation of `m` digits using the following iterative
//! algorithm:
//!
//! Find the largest `k` with `k * (m-1)! < n`. Then, the `k`th smallest (counting from 0) of the
//! `m` digits is the first in the permutation. Now take:
//!
//! `n --> n - k * (m-1)!`
//!
//! Then, the remainder of the permutation is the nth lexicographic permutation of the reamining
//! digits, which can be computed iteratively using the same algorithm.

/// The name of the problem.
pub const NAME: &'static str = "Problem 24";
/// A description of the problem.
pub const DESC: &'static str = "Lexicographic permutations";

/// Find the nth lexicographic permutation of [0, 1, ..., m - 1].
fn solve(mut n: usize, m: usize) -> Vec<usize> {
    let mut digits: Vec<usize> = (0..m).collect();
    let mut factorial: usize = (1..m).product();
    let mut permutation = Vec::with_capacity(m);

    while !digits.is_empty() {

        // Find the largest k with n > k * (m - 1)!, then add it to the permutation so far and
        // remove it from the remaining digits.
        let k = (n - 1) / factorial;
        permutation.push(digits.remove(k));

        // Update variables ready for the next iteration of this loop.
        n -= k * factorial;
        if !digits.is_empty() {
            factorial /= digits.len();
        }
    }

    permutation
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    let permutation = solve(1_000_000, 10);
    permutation.iter().fold("".to_string(), |acc, x| acc + &x.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem024() {
        assert_eq!(super::answer(), "2783915460");
    }
}
