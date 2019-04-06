//! [Problem 111 (Primes with runs)](https://projecteuler.net/problem=111)
//!
//! # Solution detail
//!
//! This is just a matter of using a careful implementation. For each digit `d`, and for each
//! `k = 10, 9, 8, ..., 0`, generate all numbers containing exactly `k` occurrences of the digit
//! `d`. Among all such numbers, find the sum of all the primes - if that sum is non-zero, then add
//! it to a running total and break; otherwise, move onto the next `k`.

use iterators::{CombinationsWithReplacement, Permutations};
use number_theory::{integer_sqrt, pow};
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Turn the given `Vec` of digits into a number
fn to_integer(digits: &[usize]) -> usize {
    digits.iter().fold(0, |acc, d| 10 * acc + d)
}

/// The sum of all of the prime `n` digit numbers in which digit `d` appears exactly `k` times.
fn repeated_digit_prime_sum(n: usize, d: usize, k: usize, sieve: &Sieve) -> usize {
    let mut result = 0;

    let other_digits = (0..10).filter(|&x| x != d);
    for spares in other_digits.combinations_with_replacement(n - k) {
        let mut digits = vec![d; k]; digits.extend(spares);
        for perm in digits.iter().map(|x| *x).permutations().filter(|ds| ds[0] != 0) {
            let number = to_integer(&perm);
            if sieve.is_prime(number as u64).unwrap() { result += number; }
        }
    }

    result
}

/// Find the sum of all primes with a given number of digits, each having a maximum number of a
/// single repeated digit.
fn solve(n: usize) -> usize {

    let mut total = 0;
    let upper_bound = pow(10, n as u64);
    let sieve = Sieve::to_limit(integer_sqrt(upper_bound));

    for d in 0..10 {
        for k in (0..n + 1).rev() {
            let subtotal = repeated_digit_prime_sum(n, d, k, &sieve);
            if subtotal > 0 {
                total += subtotal;
                break;
            }
        }
    }

    total
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10).to_string()
}

problem!(answer, "612407567715");
