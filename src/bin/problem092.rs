//! [Problem 92 (Square digit chains)](https://projecteuler.net/problem=92)
//!
//! # Solution detail
//!
//! Brute-force for this problem is possible, but would be slow. A way to speed it up is to keep
//! track of the final result for numbers that have already been checked, and shortcut the process
//! whenever we reach a number that has already been seen.
//!
//! This speeds up the runtime to an acceptable period!

use projecteuler_rs::problem;

/// Find the sum of the squares of the digits of the given number.
fn sum_of_squares(n: usize) -> usize {
    n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .map(|d| d * d)
        .sum()
}

/// Find the count of unhappy numbers below the given limit.
fn solve(limit: usize) -> usize {

    // A vector in which we keep track of the happiness of known numbers.
    let mut happinesses = vec![None; limit];

    // Now step up through the numbers, calculating the lengths of their sequences.
    for start in 1..limit {
        let (mut numbers, mut n) = (vec![start], start);
        loop {
            if n == 1 || n == 89 || happinesses[n].is_some() { break; }
            n = sum_of_squares(n);
            numbers.push(n);
        }
        let happiness = happinesses[n].or(Some(n == 1));
        numbers.iter().for_each(|&num| happinesses[num] = happiness);
    }

    happinesses.iter().filter(|&&x| x == Some(false)).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000_000).to_string()
}

problem!(answer, "8581146");
