//! [Problem 50 (Consecutive prime sum)](https://projecteuler.net/problem=50)
//!
//! # Problem statement
//!
//! The prime 41, can be written as the sum of six consecutive primes:
//!
//! 41 = 2 + 3 + 5 + 7 + 11 + 13
//!
//! This is the longest sum of consecutive primes that adds to a prime below one-hundred.
//!
//! The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21
//! terms, and is equal to 953.
//!
//! Which prime, below one-million, can be written as the sum of the most consecutive primes?
//!
//! # Solution detail
//!
//! Brute-force is good enough here - simply generate all the primes below one million, and for each
//! prime, iteratively form the sums of it with successive primes, checking if each one is prime,
//! and keeping track of the longest sequence seen so far.

#[macro_use]
extern crate projecteuler_rs;
extern crate primesieve;

use primesieve::Sieve;

/// Find the prime below n which can be written as the sum of the most successive primes.
fn solve(n: u64) -> u64 {
    let (mut best_length, mut best_total) = (0, 0);
    let sieve = Sieve::to_limit(n);
    let primes: Vec<_> = sieve.iter().collect();

    // Iterate over primes
    for (idx, &p) in primes.iter().enumerate() {
        let (mut length, mut total) = (1, p);

        // Incrementally form totals of this prime with successive primes, checking each total
        for &q in &primes[idx + 1..] {
            length += 1;
            total += q;
            if total > n {
                break;
            }

            if length > best_length && sieve.is_prime(total).unwrap() {
                best_length = length;
                best_total = total;
            }
        }
    }

    best_total
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "997651");
