//! [Problem 44 (Pentagon numbers)](https://projecteuler.net/problem=44)
//!
//! # Solution detail
//!
//! There's not much better here than brute-force. Iterate over pairs of pentagonal numbers, for
//! each one calculating the sum and the difference, and checking if both are pentagonal. Store
//! the difference of any solution, and terminate the loops when the differences grow too large
//! to be a better solution.
//!
//! The only point of note is that according to
//! [Wikipedia](https://en.wikipedia.org/wiki/Pentagonal_number#The_perfect_square_test), a way to
//! check if a number m is pentagonal is to see if 24m + 1 is a square whose square root is
//! congruent to 5 modulo 6.

use number_theory::integer_sqrt;
use projecteuler_rs::problem;

/// A function to check if a given number is pentagonal.
fn is_pentagonal(m: u64) -> bool {
    let n = 24 * m + 1;
    let sqrt = integer_sqrt(n);
    sqrt * sqrt == n && sqrt % 6 == 5
}

/// Find the smallest difference between a pair of pentagonal numbers whose sum and difference are
/// also pentagonal.
fn solve() -> u64 {

    // Somewhere to store the pentagonal numbers which are generated.
    let mut pentagonals = vec![1, 5];

    // The best difference found so far of a solution.
    let mut best_d = u64::max_value();

    // Iterate over the pentagonal numbers for as long as the difference is small enough.
    while 3 * pentagonals.len() as u64 - 2 < best_d {
        let next_pentagonal = pentagonals.last().unwrap() + 3 * pentagonals.len() as u64 + 1;
        for pentagonal in pentagonals.iter().rev() {
            let sum = next_pentagonal + pentagonal;
            let diff = next_pentagonal - pentagonal;
            if diff > best_d {
                break;
            }
            if is_pentagonal(sum) && is_pentagonal(diff) {
                best_d = diff;
            }
        }
        pentagonals.push(next_pentagonal);
    }

    best_d
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "5482660");
