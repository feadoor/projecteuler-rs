//! [Problem 132 (Large repunit factors)](https://projecteuler.net/problem=132)
//!
//! # Solution detail
//!
//! Excepting 3 as a special case, a number `p` is a factor of R(n) exactly when 10ⁿ ≡ 1 (mod p).
//! Simply iterate through primes, checking this condition (and treating 3 specially), and sum
//! the first 40 that satisfy it.

use modular_arithmetic::mod_exp;
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Determine if p is a factor of R(n)
fn is_factor_of_rn(p: u64, n: u64) -> bool {
    if p == 3 { n % 3 == 0 }
    else { mod_exp(10, n, p) == 1 }
}

/// Find the sum of the first `k` prime factors of R(n)
fn solve(n: u64, k: usize) -> u64 {
    let sieve = Sieve::to_limit(1_000_000);
    sieve.iter().filter(|&p| is_factor_of_rn(p, n)).take(k).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000_000, 40).to_string()
}

problem!(answer, "843296");
