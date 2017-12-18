//! [Problem 55 (Lychrel numbers)](https://projecteuler.net/problem=55)
//!
//! # Problem statement
//!
//! If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
//!
//! Not all numbers produce palindromes so quickly. For example,
//!
//!  - 349 + 943 = 1292
//!  - 1292 + 2921 = 4213
//!  - 4213 + 3124 = 7337
//!
//! That is, 349 took three iterations to arrive at a palindrome.
//!
//! Although no one has proved it yet, it is thought that some numbers, like 196, never produce a
//! palindrome. A number that never forms a palindrome through the reverse and add process is called
//! a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this
//! problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are
//! given that for every number below ten-thousand, it will either (i) become a palindrome in less
//! than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed
//! so far to map it to a palindrome. In fact, 10677 is the first number to be shown to require over
//! fifty iterations before producing a palindrome: 4668731596684224866951378664
//! (53 iterations, 28-digits).
//!
//! Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first
//! example is 4994.
//!
//! How many Lychrel numbers are there below ten-thousand?
//!
//! NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of
//! Lychrel numbers.
//!
//! # Solution detail
//!
//! Simply check each number up to 10,000 to see if it is a Lychrel number, remembering to use
//! arbitrary-precision integers since the numbers involved can grow very large.

#[macro_use]
extern crate projecteuler_rs;
extern crate num;

use num::{BigUint, Num};

/// Calculates the result of reversing the digits of the given number.
fn reverse(x: &BigUint) -> BigUint {
    let x_string = x.to_str_radix(10);
    let reversed_digits: String = x_string.chars().rev().collect();
    BigUint::from_str_radix(&reversed_digits, 10).unwrap()
}

/// Determines whether the given number is a possible Lychrel number, as determined by performing
/// the reverse-and-add iteration up to the given threshold number of iterations.
fn is_lychrel(x: usize, threshold: usize) -> bool {
    let start = From::from(x);
    let mut worker = reverse(&start) + start;

    for _ in 0..threshold {
        let reversed_worker = reverse(&worker);
        if reversed_worker == worker {
            return false;
        }
        worker = worker + reversed_worker;
    }

    true
}

/// Find the number of Lychrel numbers below the given limit, as determined by performing the
/// reverse-and-add iteration up to the given threshold number of iterations.
fn solve(limit: usize, threshold: usize) -> usize {
    (1..limit).filter(|&x| is_lychrel(x, threshold)).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(10_000, 50).to_string()
}

problem!(answer, "249");
