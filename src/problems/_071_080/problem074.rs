//! [Problem 74 (Digit factorial chains)](https://projecteuler.net/problem=74)
//!
//! # Solution detail
//!
//! For this problem, iterate up through the numbers 1, 2, 3, ..., 1,000,000. For each one, repeat
//! the following process:
//!
//!  - Create an empty hash set to keep track of terms in the chain so far
//!  - Repeatedly calculate the next term in the chain until:
//!      - If the next tern is in the hash set, then we have found a repeat, so can stop
//!      - If the next term is smaller than the starting value, we already know the chain length
//!        for this value, so can terminate early
//!  - Store the chain length for this starting value so that it can be used in the future as a
//!    shortcut for later values
//!
//! Simply perform this algorithm until all chain lengths have been calculated, and keep track of
//! how many chains of length 60 there are.

#[macro_use]
extern crate projecteuler_rs;

use std::collections::HashSet;

fn factorial_digit_sum(mut n: usize) -> usize {
    const DIGIT_FACTORIALS: &'static [usize] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut answer = 0;
    while n > 0 {
        answer += DIGIT_FACTORIALS[n % 10];
        n /= 10;
    }
    answer
}

fn find_chain_length(mut n: usize, previous_lengths: &[usize]) -> usize {
    let mut terms = HashSet::new();
    while !terms.contains(&n) {
        if n < previous_lengths.len() {
            return terms.len() + previous_lengths[n];
        }
        terms.insert(n);
        n = factorial_digit_sum(n);
    }

    terms.len()
}

/// Find how many numbers below the given limit form a chain of the given number of terms before
/// repeating themselves.
fn solve(limit: usize, required_length: usize) -> usize {
    let mut length_cache = vec![0];
    let mut count = 0;

    for n in 1..limit {
        let chain_length = find_chain_length(n, &length_cache);
        if chain_length == required_length {
            count += 1;
        }
        length_cache.push(chain_length);
    }

    count
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000, 60).to_string()
}

problem!(answer, "402");
