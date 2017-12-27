//! [Problem 69 (Totient maximum)](https://projecteuler.net/problem=69)
//!
//! # Problem statement
//!
//! Euler's Totient function, ϕ(n) [sometimes called the phi function], is used to determine the
//! number of numbers less than n which are relatively prime to n. For example, as 1, 2, 4, 5, 7,
//! and 8, are all less than nine and relatively prime to nine, ϕ(9) = 6.
//!
//! | n  | Relatively prime | ϕ(n) | n / ϕ(n)  |
//! |----|------------------|------|-----------|
//! | 2  | 1                | 1    | 2         |
//! | 3  | 1, 2             | 2    | 1.5       |
//! | 4  | 1, 3             | 2    | 2         |
//! | 5  | 1, 2, 3, 4       | 4    | 1.25      |
//! | 6  | 1, 5             | 2    | 3         |
//! | 7  | 1, 2, 3, 4, 5, 6 | 6    | 1.1666... |
//! | 8  | 1, 3, 5, 7       | 4    | 2         |
//! | 9  | 1, 2, 4, 5, 7, 8 | 6    | 1.5       |
//! | 10 | 1, 3, 7, 9       | 4    | 2.5       |
//!
//! It can be seen that n = 6 produces a maximum n / ϕ(n) for n ≤ 10.
//!
//! Find the value of n ≤ 1,000,000 for which n / ϕ(n) is a maximum.
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
