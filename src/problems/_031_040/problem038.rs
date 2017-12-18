//! [Problem 38 (Pandigital multiples)](https://projecteuler.net/problem=38)
//!
//! # Problem statement
//!
//! Take the number 192 and multiply it by each of 1, 2, and 3:
//!
//! 192 × 1 = 192
//!
//! 192 × 2 = 384
//!
//! 192 × 3 = 576
//!
//! By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576
//! the concatenated product of 192 and (1,2,3)
//!
//! The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the
//! pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
//!
//! What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated
//! product of an integer with (1,2, ... , n) where n > 1?
//!
//! # Solution detail
//!
//! Since the final answer will have 9 digits, the base number that the product is formed from can
//! have at most 4 digits. This is not many numbers to check, so for each one, form the smallest
//! concatenated product that has at least 9 digits, then check to see if it is pandigital or not.
//! Then just remember the largest solution that is found during this process.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use number_theory::pow;

/// Check whether the given number is pandigital.
fn is_pandigital(mut num: u64) -> bool {

    // Keep track of the digits seen so far.
    let mut seen = vec![false; 10];

    // Check the number's digits and mark them as seen. Return false if we have already seen one
    // of the digits at any point.
    while num != 0 {
        let digit = num % 10;
        num /= 10;
        if seen[digit as usize] {
            return false;
        } else {
            seen[digit as usize] = true;
        }
    }

    // We need to have seen every digit except zero.
    !seen[0] && seen[1..].iter().all(|&x| x)
}

/// Find the number of digits in the given number.
fn num_digits(n: u64) -> u64 {
    (n as f64).log(10.0).floor() as u64 + 1
}

/// Concatenate the two given numbers, `a` followed by `b`.
fn concatenate(a: u64, b: u64) -> u64 {
    let digits = num_digits(b);
    a * pow(10, digits) + b
}

/// Find the first concatenated product of n which contains at least 9 digits.
fn concatenated_product(mut n: u64) -> u64 {
    for multiplier in 2.. {
        if num_digits(n) < 9 {
            n = concatenate(n, multiplier * n);
        } else {
            break;
        }
    }

    n
}

/// Find the largest pandigital concatenated product.
fn solve() -> u64 {

    // Keep track of the best product found so far.
    let mut best = 0;

    // Iterate over all base numbers, and check if the concatenated product is pandigital.
    for base in 1..10000 {
        let product = concatenated_product(base);
        if product > best && is_pandigital(product) {
            best = product;
        }
    }

    best
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "932718654");
