//! [Problem 14 (Longest Collatz sequence)](https://projecteuler.net/problem=14)
//!
//! # Problem statement
//!
//! The following iterative sequence is defined for the set of positive integers:
//!
//! n → n/2 (n is even)
//! n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//!
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
//! Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
//! finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.
//!
//! # Solution detail
//!
//! The naive solution would be to iterate up through the numbers 1, 2, ..., 1000000, and for each
//! one, step through its Collatz sequence, find the length, and keep track of the best.
//!
//! This, unfortunately, repeats a lot of work, since lots of numbers share portions of their
//! sequences with other numbers. The better solution is to remember, for each number, how long its
//! Collatz sequence is. Then, for larger numbers, when we reach a point in the sequence where the
//! number has gone below its original value, we will already know how many steps remain.

/// The name of the problem.
pub const NAME: &'static str = "Problem 14";
/// A description of the problem.
pub const DESC: &'static str = "Longest Collatz sequence";

/// Find the number below the given limit with the longest Collatz sequence.
fn solve(limit: usize) -> usize {
    // A vector in which we keep track of the lengths of previously-seen sequence.
    let mut collatz_lens = vec![0; limit];

    // Now step up through the numbers, calculating the lengths of their sequences.
    for start in 2..limit {
        let mut n = start;
        for steps in 1.. {
            n = if n & 1 == 0 { n / 2 } else { 3 * n + 1 };
            if n < start {
                collatz_lens[start] = steps + collatz_lens[n];
                break;
            }
        }
    }

    (0..limit).max_by_key(|&x| collatz_lens[x]).unwrap()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1000000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem014() {
        assert_eq!(super::answer(), "837799");
    }
}
