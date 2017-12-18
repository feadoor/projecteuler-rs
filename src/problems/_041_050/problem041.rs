//! [Problem 41 (Pandigital prime)](https://projecteuler.net/problem=41)
//!
//! # Problem statement
//!
//! We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
//! exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
//!
//! What is the largest n-digit pandigital prime that exists?
//!
//! # Solution detail
//!
//! Note that any 9- or 8-digit pandigital number is divisible by 3 and hence not prime. We
//! therefore only need to check the 7-digit pandigitals and smaller. Simply iterate over them in
//! reverse order, checking each one for primality and stopping when we find one.

#[macro_use]
extern crate projecteuler_rs;
extern crate permutohedron;
extern crate primesieve;

use permutohedron::LexicalPermutation;
use primesieve::Sieve;

/// Convert a vector of digits into an actual number.
fn to_int(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |acc, x| 10 * acc + x)
}

/// A structure for iterating over the pandigital numbers of a given length in reverse order.
struct PandigitalIterator {
    digits: Vec<u64>,
    next_num: Option<u64>,
}

impl Iterator for PandigitalIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let curr_num = self.next_num;
        if self.digits.prev_permutation() {
            self.next_num = Some(to_int(&self.digits));
        } else {
            self.next_num = None;
        }
        curr_num
    }
}

/// An iterator over the pandigital numbers of the given length, in reverse order.
fn pandigitals(num_digits: u64) -> PandigitalIterator {
    let digits = (1..num_digits + 1).rev().collect::<Vec<u64>>();
    let first_num = to_int(&digits);

    PandigitalIterator {
        digits: digits,
        next_num: Some(first_num),
    }
}

/// Find the largest pandigital prime.
fn solve() -> u64 {

    // Set up a prime sieve large enough to test 7-digit numbers for primality.
    let sieve = Sieve::to_limit(10_000);

    // Iterate over the pandigitals and check for primality.
    for num_digits in (1..8).rev() {
        for num in pandigitals(num_digits) {
            if sieve.is_prime(num).unwrap() {
                return num;
            }
        }
    }

    0
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "7652413");
