//! [Problem 112 (Bouncy numbers)](https://projecteuler.net/problem=112)
//!
//! # Solution detail
//!
//! Brute force works here! Use the naive method to check each number in turn, and keep track of how
//! many bouncy numbers there are. Stop when the 99% proportion is found.

use projecteuler_rs::problem;

/// Check if the number has its digits in increasing order
fn is_increasing(mut n: usize) -> bool {
    let mut last_digit = n % 10; n /= 10;
    while n > 0 {
        let curr_digit = n % 10; n /= 10;
        if curr_digit > last_digit { return false; }
        last_digit = curr_digit;
    }
    true
}

/// Check if the number has its digits in decreasing order
fn is_decreasing(mut n: usize) -> bool {
    let mut last_digit = n % 10; n /= 10;
    while n > 0 {
        let curr_digit = n % 10; n /= 10;
        if curr_digit < last_digit { return false; }
        last_digit = curr_digit;
    }
    true
}

/// Check if the given number is bouncy
fn is_bouncy(n: usize) -> bool {
    !(is_increasing(n) || is_decreasing(n))
}

/// Find the first number `n` for which the proportion of bouncy numbers at most `n` is exactly the
/// given percentage.
fn solve(percentage: usize) -> usize {
    let (mut bouncy_count, mut n) = (0, 1);
    while n * percentage != bouncy_count * 100 {
        n += 1;
        if is_bouncy(n) { bouncy_count += 1; }
    }
    n
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(99).to_string()
}

problem!(answer, "1587000");
