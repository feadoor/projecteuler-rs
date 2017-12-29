//! [Problem 36 (Double-base palindromes)](https://projecteuler.net/problem=36)
//!
//! # Solution detail
//!
//! There are fewer base-10 palindromes less than one million than there are binary palindromes.
//! This means that we can simply iterate over the base-10 palindromes, and for each one, check if
//! it is also a binary palindrome.
//!
//! To iterate over the base-10 palindromes, we can simply iterate over all possible 'first halfs'
//! of numbers, and create palindromes from them by reversing and gluing together.

#[macro_use]
extern crate projecteuler_rs;
#[macro_use]
extern crate itertools;
extern crate number_theory;

use number_theory::pow;

/// Check whether the given number is a binary palindrome.
fn is_binary_palindrome(mut num: u64) -> bool {

    // First find the binary digits of the number.
    let mut binary_digits = Vec::new();
    while num != 0 {
        binary_digits.push(num & 1);
        num >>= 1;
    }

    // Now just check if the digits read the same forwards and backwards.
    binary_digits.iter().eq(binary_digits.iter().rev())
}

/// Convert a number into an even-length palindrome by reversing and concatenating.
fn even_palindromise(mut n: u64) -> u64 {
    let mut m = n;
    while m != 0 {
        n = 10 * n + m % 10;
        m /= 10;
    }
    n
}

/// Convert a number into an odd-length palindrome with middle digit d by reversing and
/// concatenating.
fn odd_palindromise(mut n: u64, d: u64) -> u64 {
    let mut m = n;
    n = 10 * n + d;
    while m != 0 {
        n = 10 * n + m % 10;
        m /= 10;
    }
    n
}

/// An iterator over all palindromes with up to the given number of digits in base 10.
fn palindromes(digits: u64) -> Box<Iterator<Item = u64>> {

    // Even-length palindromes.
    let max_length = digits / 2;
    let even_palindromes = (1..pow(10, max_length)).map(even_palindromise);

    // Odd-length palindromes
    let max_length = (digits - 1) / 2;
    let odd_palindromes = iproduct!(1..pow(10, max_length), 0..10)
        .map(|(n, d)| odd_palindromise(n, d));

    // Single-digit palindromes
    let single_digit_palindromes = if digits > 0 { 0..10 } else { 0..0 };

    Box::new(single_digit_palindromes.chain(odd_palindromes).chain(even_palindromes))
}

/// Find the sum of all numbers up to the given number of digits which are palindromic in base 2
/// and in base 10.
fn solve(digits: u64) -> u64 {
    palindromes(digits).filter(|&x| is_binary_palindrome(x)).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(6).to_string()
}

problem!(answer, "872187");
