//! [Problem 69 (Totient maximum)](https://projecteuler.net/problem=69)
//!
//! # Solution detail
//!
//! The product formula for ϕ(n) tells us that n / ϕ(n) is equal to the product, over all primes p
//! dividing n, of the quantity p / (p - 1).
//!
//! This quantity is decreasing as p gets larger, so n / ϕ(n) is maximised when n is the product of
//! as many small primes as possible - that is, n = 2, 2 × 3, 2 × 3 × 5... and so on.
//!
//! We can therefore just generate the first k primes, where k is chosen to be large enough that
//! 2<sup>k</sup> ≥ 1,000,000, and calculate the product of the smallest primes until it would
//! exceed 1,000,000.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;

use primesieve::Sieve;

/// Find the value of n not exceeding the given limit for which n / ϕ(n) is maximised.
fn solve(limit: u64) -> u64 {
    let number_of_primes = (limit as f64).log(2.0).ceil() as usize;
    let sieve = Sieve::to_n_primes(number_of_primes);

    let mut ans = 1;
    for p in sieve.iter() {
        if ans * p <= limit {
            ans *= p;
        } else {
            return ans;
        }
    }

    unreachable!()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "510510");
