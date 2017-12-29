//! [Problem 63 (Powerful digit counts)](https://projecteuler.net/problem=63)
//!
//! # Solution detail
//!
//! Since 10<sup>n</sup> has n + 1 digits, there are no solutions a<sup>n</sup> with a > 9.
//!
//! For 1 ≤ a ≤ 9, if a<sup>n</sup> has fewer than n digits, then also a<sup>n+1</sup> has fewer
//! than n + 1 digits, so all the solutions are of the form a<sup>n</sup> for some consecutive range
//! of values of n.
//!
//! We therefore just need to iterate over each 1 ≤ a ≤ 9, and for each one, find the first power
//! with fewer than the requisite number of digits - this will be sufficient to have found all
//! solutions.

#[macro_use]
extern crate projecteuler_rs;
extern crate num;

use num::BigUint;

/// Find the number of nth powers which contain exactly n digits.
fn solve() -> usize {
    let mut count = 0;

    for a in 1..10usize {
        let mut power_of_a = BigUint::from(a);
        let mut power_of_ten = BigUint::from(1usize);

        while power_of_a >= power_of_ten {
            count += 1;
            power_of_a *= a;
            power_of_ten *= 10usize;
        }
    }

    count
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "49");
