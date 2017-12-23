//! [Problem 65 (Convergents of e)](https://projecteuler.net/problem=65)
//!
//! # Problem statement
//!
//! The square root of 2 can be written as an infinite continued fraction √2 = [1; (2)] - (2)
//! indicates that 2 repeats ad infinitum. In a similar way, √23 = [4; (1, 3, 1, 8)].
//!
//! It turns out that the sequence of partial values of continued fractions for square roots provide
//! the best rational approximations. The sequence of the first ten convergents for √2 are:
//!
//! 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, ...
//!
//! What is most surprising is that the important mathematical constant, e, has a continued fraction
//! representation given by:
//!
//! e = [2; 1,2,1, 1,4,1, 1,6,1 , ... , 1,2k,1, ...].
//!
//! The first ten terms in the sequence of convergents for e are:
//!
//! 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...
//!
//! The sum of digits in the numerator of the 10th convergent is 1 + 4 + 5 + 7 = 17.
//!
//! Find the sum of digits in the numerator of the 100th convergent of the continued fraction for e.
//!
//! # Solution detail
//!
//! Simply calculate the 100th convergent and sum its digits - nothing more clever than that!

#[macro_use]
extern crate projecteuler_rs;
extern crate continued_fractions;

use continued_fractions::ContinuedFractionConvergents;

/// An iterator over the terms of the continued fraction expansion of e.
struct EContinuedFractionIterator {
    started: bool,
    next_even_term: u64,
    idx_in_group: u64,
}

impl EContinuedFractionIterator {
    fn new() -> EContinuedFractionIterator {
        EContinuedFractionIterator {
            started: false,
            next_even_term: 2,
            idx_in_group: 0,
        }
    }
}

impl Iterator for EContinuedFractionIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if !self.started {
            self.started = true;
            return Some(2);
        }

        let next = match self.idx_in_group {
            1 => self.next_even_term,
            _ => 1,
        };

        self.idx_in_group += 1;
        if self.idx_in_group == 3 {
            self.idx_in_group = 0;
            self.next_even_term += 2;
        }

        Some(next)
    }
}

/// Find the sum of the digits of the numerator of the given convergent of the continued fraction
/// expansion of e.
fn solve(n: usize) -> u64 {
    ContinuedFractionConvergents::new(EContinuedFractionIterator::new())
        .skip(n - 1)
        .next().unwrap().0
        .to_str_radix(10).as_bytes().iter()
        .map(|x| (x - b'0') as u64)
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100).to_string()
}

problem!(answer, "272");
