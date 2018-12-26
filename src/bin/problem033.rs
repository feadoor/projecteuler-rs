//! [Problem 33 (Digit cancelling fractions)](https://projecteuler.net/problem=33)
//!
//! # Solution detail
//!
//! There are four cases to consider, depending on which digit is cancelled from each side:
//!
//! ```text
//! 10a + b / 10a + d = b / d
//! 10a + b / 10c + b = a / c
//! 10a + b / 10b + d = a / d
//! 10a + b / 10c + a = b / c
//! ```
//!
//! Expanding out each of these equations, we get to the following:
//!
//! ```text
//! a(b - d) = 0
//! b(a - c) = 0
//! d(9a + b) = 10ab
//! b(9c + a) = 10ac
//! ```
//!
//! Of these, the first two represent trivial solutions, and the last two represent the same set
//! of solutions, but with the two halves of the fraction switched. This means we can focus only
//! on the third of these equations.
//!
//! It is easy to brute-force solutions to this equation by iterating over `a, b` and checking if
//! this gives an integer value for `d`.

use std::cmp::Ordering;
use number_theory::gcd;
use projecteuler_rs::problem;

/// Find the denominator of the product of all 2-digit cancelling fractions.
fn solve() -> u64 {

    // The numerator and denominator of the final answer.
    let mut final_num = 1;
    let mut final_denom = 1;

    // Loop through all choices for a and b.
    for a in 1..10 {
        for b in 1..10 {

            // Calculate d and check if we have a solution.
            if (10 * a * b) % (9 * a + b) == 0 {
                let d = (10 * a * b) / (9 * a + b);
                if d > 0 && d < 10 {

                    // We have a solution, so update the numerator and denominator with the new
                    // solution. Note that if a == b then the solution is trivial and should be
                    // ignored.
                    match a.cmp(&b) {
                        Ordering::Less => {
                            final_num *= 10 * a + b;
                            final_denom *= 10 * b + d;
                        }
                        Ordering::Greater => {
                            final_num *= 10 * b + d;
                            final_denom *= 10 * a + b;
                        }
                        Ordering::Equal => (),
                    }
                }
            }
        }
    }

    // Cancel the final solution down to lowest terms.
    final_denom / gcd(final_num, final_denom)
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "100");
