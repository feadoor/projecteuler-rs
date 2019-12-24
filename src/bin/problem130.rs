//! [Problem 130 (Composites with prime repunit property)](https://projecteuler.net/problem=130)
//!
//! # Solution detail
//!
//! It is clear that R(k) is given by the formula R(k) = (10ᵏ - 1) / 9. That means that A(n) is the
//! least value of k for which 10ᵏ - 1 is divisible by 9n - in other words, the order of 10 modulo
//! 9n. It follows that A(n) divides n - 1 exactly when 10⁽ⁿ⁻¹⁾ ≡ 1 (mod 9n).
//!
//! So simply iterate up through values of n, checking for this condition and discarding the primes.

use modular_arithmetic::mod_exp;
use number_theory::is_prime;
use projecteuler_rs::problem;

/// Find the sum of the first `k` composite values of `n` for which A(n) divides n - 1
fn solve(k: usize) -> u64 {
    (2..).filter(|&n| mod_exp(10, n - 1, 9 * n) == 1 && !is_prime(n)).take(k).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(25).to_string()
}

problem!(answer, "149253");
