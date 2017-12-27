//! [Problem 40 (Champernowne's constant)](https://projecteuler.net/problem=40)
//!
//! # Problem statement
//!
//! An irrational decimal fraction is created by concatenating the positive integers:
//!
//! `0.123456789101112131415161718192021...`
//!
//! It can be seen that the 12th digit of the fractional part is 1.
//!
//! If d<sub>n</sub> represents the nth digit of the fractional part, find the value of the
//! following expression.
//!
//! d<sub>1</sub> × d<sub>10</sub> × d<sub>100</sub> × d<sub>1000</sub> ×
//! d<sub>10000</sub> × d<sub>100000</sub> × d<sub>1000000</sub>
//!
//! # Solution detail
//!
//! We need a quick way of finding the digit at a given index `n` in Champernowne's constant.
//! Define `f(k)` to be the number of digits used when writing out the numbers from 1 to k. Then
//! find the smallest k with `f(k) >= n`. This means that we are in the process of writing the
//! number k when we write the nth digit, and we can count through the digits of k itself to
//! find the exact digit.
//!
//! Finding k is easily accomplished using a binary search, and evaluating `f(k)` is a simple
//! combinatorial problem.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;
extern crate search;

use number_theory::pow;
use search::binary_search;

/// Calculate the number of digits used when writing the numbers from 1 to k.
fn digits_used(k: u64) -> u64 {

    if k == 0 {
        0
    } else {
        // First find the number of digits in `k`.
        let digits = (k as f64).log(10.0).floor() as u64 + 1;

        // For each possible number of digits less than the length of k, we use d * 9 * 10^(d-1)
        // digits in writing all numbers of length d.
        let mut ans = 0;
        let mut power = 1;
        for d in 1..digits {
            ans += d * 9 * power;
            power *= 10;
        }

        // We have also written (k + 1 - 10 ^ (digits - 1)) numbers of the same length as k.
        ans + digits * (k + 1 - power)
    }
}

/// Find the digit at the specified index of CHampernowne's constant.
fn find_digit(index: u64) -> u64 {

    // Find the number currently being written and how far we need to go through it.
    let current_num = binary_search(&digits_used, index);
    let digits_left = digits_used(current_num) - index;

    // Extract the right digit from the number.
    (current_num / pow(10, digits_left)) % 10
}

/// Find the product of the digits at the specified positions in Champernowne's constant.
fn solve(indices: &[u64]) -> u64 {
    indices.iter().map(|&x| find_digit(x)).product()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(&[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]).to_string()
}

problem!(answer, "210");
