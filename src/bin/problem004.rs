//! [Problem 4 (Largest palindrome product)](https://projecteuler.net/problem=4)
//!
//! # Solution detail
//!
//! Simply form all possible products of 2-digit numbers, and keep track of the largest one which
//! is a palindrome. As a simple optimisation, we can consider the larger products first, and
//! break out of loops early when all future products will be smaller than the current best.

use projecteuler_rs::problem;

/// Check whether the given number is a palindrome.
fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s.chars().eq(s.chars().rev())
}

/// Find the largest palindrome which is the product of two numbers between lower (inclusive) and
/// upper (exclusive).
fn solve(lower: u64, upper: u64) -> u64 {

    // Loop over all pairs of numbers, checking if the products are palindromes. Break early if
    // the future products will be too small.
    let mut best = 0;
    for x in (lower..upper).rev() {
        if x * x < best { break; }
        for y in (lower..x + 1).rev() {
            let product = x * y;
            if product < best { break; }
            if is_palindrome(product) { best = product; }
        }
    }

    best

}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100, 1_000).to_string()
}

problem!(answer, "906609");
