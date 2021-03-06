//! [Problem 21 (Amicable numbers)](https://projecteuler.net/problem=21)
//!
//! # Solution detail
//!
//! We will check each number up to 10000 in turn to see if it is amicable. To do this, it will be
//! necessary to be able to compute `d(n)` for given `n`. Then, to check if n is amicable, we just
//! check that `d(d(n)) = n` and that `d(n) ≠ n`.
//!
//! To calculate the sum of the divisors of a given number, it will be necessary to be able to
//! fully factorise that number, which requires having sieved primes up to the square root of that
//! number. It would be good to know, in advance, the largest number that we have to factorise, so
//! that we can presieve the primes just once. The largest number we might see is the maximum
//! value of `d(n)` for `n ≤ 10000`.
//!
//! Thankfully, under the assumption of the Riemann Hypothesis, Robin's Theorem gives us an upper
//! bound on this maximum. It says that for all n > 5040, `d(n) < e^γ * log(log(n)) * n`. Since
//! for n ≤ 5040, `d(n) < 4n`, this means that we need the highest number we might need to be able
//! to factorise is:
//!
//! `max(4, e^γ * log(log(10000))) * 10000`

use std::f64::consts;
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Check whether n is amicable.
fn is_amicable(n: u64, sieve: &Sieve) -> bool {
    let dn = sieve.sum_of_divisors(n).unwrap() - n;
    (n != dn) && (n == sieve.sum_of_divisors(dn).unwrap() - dn)
}

/// Find the sum of the amicable numbers below the given limit.
fn solve(lim: u64) -> u64 {
    // Use Robin's Theorem to get an upper bound on how many primes need to be sieved.
    static GAMMA: f64 = 0.57721566490153286060651209008240243104215933593992;
    let scale_factor = f64::max(4.0, consts::E.powf(GAMMA) * (lim as f64).ln().ln());
    let sieve_limit = (scale_factor * (lim as f64)).ceil().sqrt() as u64;

    // Sieve for primes up to the required limit and check for amicable numbers.
    let sieve = Sieve::to_limit(sieve_limit);
    (2..lim).filter(|&x| is_amicable(x, &sieve)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000).to_string()
}

problem!(answer, "31626");
