//! [Problem 47 (Distinct primes factors)](https://projecteuler.net/problem=47)
//!
//! # Solution detail
//!
//! Simply iterate up through all positive integers, factorising each one, and stopping when four
//! consecutive numbers each have four distinct prime factors.
//!
//! To aid factorisation, pre-sieve a number of primes, and sieve more when we encounter a number
//! that cannot be factorised with the current list of primes.

use primesieve::Sieve;
use projecteuler_rs::problem;

// Factorise the number using the given sieve, expanding to a higher limit if necessary, and
// return whether or not the number has at least n prime factors.
fn has_at_least_n_prime_factors(n: usize, number: u64, sieve: &mut Sieve) -> bool {
    let mut factors = sieve.factorise(number);
    while factors.is_err() {
        *sieve = Sieve::to_limit(2 * sieve.limit());
        factors = sieve.factorise(number);
    }

    factors.unwrap().len() >= n
}

/// Find the first n consecutive numbers to have at least n prime factors.
fn solve(n: usize) -> u64 {
    let mut consecutive_successes = 0;
    let mut sieve = Sieve::to_limit(1000);

    for number in 2.. {
        if has_at_least_n_prime_factors(n, number, &mut sieve) {
            consecutive_successes += 1;
            if consecutive_successes == n {
                return number - (n as u64 - 1);
            }
        } else {
            consecutive_successes = 0;
        }
    }

    unreachable!()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(4).to_string()
}

problem!(answer, "134043");
