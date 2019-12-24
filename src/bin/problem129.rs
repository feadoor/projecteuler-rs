//! [Problem 129 (Repunit divisibility)](https://projecteuler.net/problem=129)
//!
//! # Solution detail
//!
//! It is clear that R(k) is given by the formula R(k) = (10ᵏ - 1) / 9. That means that A(n) is the
//! least value of k for which 10ᵏ - 1 is divisible by 9n - in other words, the order of 10 modulo
//! 9n.
//!
//! Since 10ᵏ - 1 is always divisible by 9, this order is at most n, since that is the maximum
//! number of distinct values that can be taken by the powers of 10 modulo 9n.
//!
//! Therefore, to find `n` with A(n) at least a particular number `k`, we just need to begin the
//! search at n = k, and keep going until we find one that works.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the smallest value of `n` for which A(n) exceeds the given value.
fn solve(k: u64) -> u64 {
    let sieve = Sieve::to_limit(2 * k);
    for n in (k..).filter(|&n| n % 2 != 0 && n % 5 != 0) {
        if sieve.order(10, 9 * n).unwrap() >= k {
            return n;
        }
    }

    unreachable!()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "1000023");
