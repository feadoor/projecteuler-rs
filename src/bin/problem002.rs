//! [Problem 2 (Even Fibonacci numbers)](https://projecteuler.net/problem=2)
//!
//! # Solution detail
//!
//! Starting from 2, every third Fibonacci number is even, as can be seen by considering the
//! sequence modulo 2. This means we can calculate the sum simply by taking every third Fibonacci
//! number until the terms exceed four million and taking the sum as we go along.

use projecteuler_rs::problem;

/// A structure which will allow iteration over Fibonacci numbers.
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

/// Returns a Fibonacci sequence iterator starting 1, 1, 2, 3, ...
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

/// Find the sum of all even Fibonacci numbers below the given limit.
fn solve(limit: u64) -> u64 {
    fibonacci().skip(2).step_by(3)
        .take_while(|&x| x <= limit)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(4_000_000).to_string()
}

problem!(answer, "4613732");
