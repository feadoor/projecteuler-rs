//! [Problem 17 (Number letter counts)](https://projecteuler.net/problem=17)
//!
//! # Solution detail
//!
//! This is just a case of carefully implementing the algorithm that humans use when working out
//! how to say a number out loud. Once that's done, simply add up the lengths of each number from
//! 1 to 1000.

use projecteuler_rs::problem;

// Numbers below 20 are special, so store their lengths.
const SMALL_LENS: &'static [usize; 21] =
    &[0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8, 6];

/// Find the number of letters of a number below 100.
fn number_of_letters_below_100(n: usize) -> usize {
    assert!(1 <= n && n <= 100);

    match n {
        1..=20 => SMALL_LENS[n],
        _ => {
            let prefix = match n / 10 {
                2 => "twenty".len(),
                3 => "thirty".len(),
                4 => "forty".len(),
                5 => "fifty".len(),
                6 => "sixty".len(),
                7 => "seventy".len(),
                8 => "eighty".len(),
                9 => "ninety".len(),
                _ => unreachable!(),
            };

            prefix + SMALL_LENS[n % 10]
        }
    }
}

/// Find the number of letters in the number 1 <= n <= 1000.
fn number_of_letters_below_1000(n: usize) -> usize {
    assert!(1 <= n && n <= 1000);

    // If n is equal to 1000, just return the answer straight away.
    match n {
        1000 => "onethousand".len(),
        100..=999 => {
            let prefix = "hundred".len() + SMALL_LENS[n / 100];
            if n % 100 == 0 {
                prefix
            } else {
                prefix + "and".len() + number_of_letters_below_100(n % 100)
            }
        }
        1..=99 => number_of_letters_below_100(n),
        _ => unreachable!(),
    }
}

/// Find the total number of letters used when writing the numbers up to n.
fn solve(n: usize) -> usize {
    (1..n + 1).map(number_of_letters_below_1000).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "21124");
