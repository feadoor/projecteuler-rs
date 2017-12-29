//! [Problem 23 (Non-abundant sums)](https://projecteuler.net/problem=23)
//!
//! # Solution detail
//!
//! See [here](http://mathschallenge.net/full/sum_of_two_abundant_numbers) for a proof that all
//! numbers greater than 28123 are the sum of two abundant numbers.
//!
//! To find the numbers which are not the sum of two abundant numbers, therefore, it will be
//! enough to check every number up to 28123, and store whether each number is abundant in a
//! boolean array. Then, to see if a given number `n` is a sum of two abundant numbers, it will be
//! enough to iterate over the abundant numbers `x` and check if `n - x` is abundant.
//!
//! The abundant numbers themselves can be found by brute force, using prime factorisation to
//! calculate the sum of divisors.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;
extern crate primesieve;

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// Check whether n is a sum of two abundant numbers.
fn is_sum_of_abundants(n: usize, abundants: &[bool]) -> bool {
    for abundant in 1..abundants.len() {
        if n < abundant + abundant {
            break;
        } else if abundants[abundant] && abundants[n - abundant] {
            return true;
        }
    }

    false
}

/// Find the abundant numbers up to the given limit.
fn abundants(lim: u64) -> Vec<bool> {
    // First, sieve out all the prime numbers needed to factorise numbers up to the limit.
    let sieve = Sieve::to_limit(integer_sqrt(lim));

    // Now factorise each number and store the abundant ones.
    (0..lim).map(|x| x != 0 && sieve.sum_of_divisors(x).unwrap() > x + x).collect()
}

/// Find the sum of the numbers below the given limit which are not a sum of two abundant numbers.
fn solve(lim: u64) -> u64 {
    let abundants = abundants(lim);
    (1..lim as usize).filter(|&x| !is_sum_of_abundants(x, &abundants)).sum::<usize>() as u64
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(28_123).to_string()
}

problem!(answer, "4179871");
