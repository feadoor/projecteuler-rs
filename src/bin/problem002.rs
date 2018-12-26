//! [Problem 2 (Even Fibonacci numbers)](https://projecteuler.net/problem=2)
//!
//! # Solution detail
//!
//! Starting from 2, every third Fibonacci number is even, as can be seen by considering the
//! sequence modulo 2. This means we can calculate the sum simply by taking every third Fibonacci
//! number until the terms exceed four million and taking the sum as we go along.

#![feature(generators)]

use generators::GeneratorIteratorAdapter;
use itertools::Itertools;
use projecteuler_rs::problem;

/// Returns a Fibonacci sequence iterator starting 1, 1, 2, 3, ...
fn fibonacci() -> impl Iterator<Item = u64> {
    GeneratorIteratorAdapter::of(|| {
        let (mut a, mut b) = (0, 1);
        loop {
            let new_b = a + b;
            a = b; b = new_b;
            yield a
        }
    })
}

/// Find the sum of all even Fibonacci numbers below the given limit.
fn solve(limit: u64) -> u64 {
    fibonacci().skip(2).step(3)
        .take_while(|&x| x <= limit)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(4_000_000).to_string()
}

problem!(answer, "4613732");
