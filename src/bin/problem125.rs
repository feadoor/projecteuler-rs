//! [Problem 125 (Palindromic sums)](https://projecteuler.net/problem=125)
//!
//! # Solution detail
//!
//! It's doubtful that there is anything more effective than brute force for this problem, although
//! we can still be careful with the implementation. The easiest way will be to generate all of the
//! possible sums of consecutive squares, and for each one, check if it is a palindrome.
//!
//! There are two points to be careful of. Firstly, make sure that no number is counted twice if it
//! is a sum of consecutive squares in more than one way. Secondly, we should avoid repeated work
//! when generating the square sums. An easy way to achieve this is to store the sums of consecutive
//! squares up to a certain limit, and add a single square onto those partial sums when calculating
//! the next group.

use std::collections::HashSet;
use projecteuler_rs::problem;

/// Calculate the set of numbers less than the given limit which are a sum of consecutive squares.
fn sums_of_consecutive_squares(limit: u64) -> HashSet<u64> {
    let (mut sums, mut numbers) = (vec![1], HashSet::new());
    for square in (2..).map(|x| x * x).take_while(|&sq| sq < limit) {
        for sum in sums.iter_mut() {
            *sum += square;
            if *sum < limit { numbers.insert(*sum); }
        }
        sums.push(square);
    }
    numbers
}

/// Check whether the given number is a palindrome.
fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s.chars().eq(s.chars().rev())
}

/// Find the sum of all palindromic consecutive-square-sums below the given limit.
fn solve(limit: u64) -> u64 {
    sums_of_consecutive_squares(limit).iter().filter(|&&x| is_palindrome(x)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100_000_000).to_string()
}

problem!(answer, "2906969179");
