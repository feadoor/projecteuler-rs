//! [Problem 133 (Repunit nonfactors)](https://projecteuler.net/problem=133)
//!
//! # Solution detail
//!
//! The repunit R(k) = (10ᵏ - 1) / 9 is divisible by a prime `p` (greater than 3) exactly when
//! 10ᵏ ≡ 1 (mod p) - that is, when k is a multiple of the order of 10 modulo p.
//!
//! When considering k = 10ⁿ, as in this problem, this will be the case for some n exactly when the
//! order of 10 modulo p only has 2 and 5 as its prime factors. That means we can just check every
//! prime, calculate the orders and sum those that don't satisfy the condition. Skip the primes
//! 2, 3 and 5 as special cases and remember to add 10 onto the total at the end.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Determine if `n` is divisible by the primes 2 and 5 and no others.
fn divisible_only_by_2_and_5(mut n: u64) -> bool {
    while n % 2 == 0 { n /= 2; }
    while n % 5 == 0 { n /= 5; }
    n == 1
}

/// Find the sum of the first `k` prime factors of R(n)
fn solve(limit: u64) -> u64 {
    let sieve = Sieve::to_limit(limit);
    10 + sieve.iter().skip(3)
        .take_while(|&p| p < limit)
        .filter(|&p| !divisible_only_by_2_and_5(sieve.order(10, p).unwrap()))
        .sum::<u64>()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100_000).to_string()
}

problem!(answer, "453647705");
