//! [Problem 76 (Counting summations)](https://projecteuler.net/problem=76)
//!
//! # Solution detail
//!
//! Simply generate the sequence of partition numbers until the required 100th term has been
//! generated.
//!
//! Use Euler's [Pentagonal number theorem](https://en.wikipedia.org/wiki/Pentagonal_number_theorem#Relation_with_partitions)
//! to efficiently generate the sequence of partition numbers.

use number_theory::partition_numbers;
use projecteuler_rs::problem;

/// Find the nth partition number.
fn solve(n: usize) -> u64 {
    partition_numbers::<u64>().skip(n).next().unwrap() - 1
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "190569291");
