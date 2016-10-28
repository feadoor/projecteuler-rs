//! [Problem 26 (Reciprocal cycles)](https://projecteuler.net/problem=26)
//!
//! # Problem statement
//!
//! A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
//! with denominators 2 to 10 are given:
//!
//! 1/2 =   0.5<br>
//! 1/3 =   0.(3)<br>
//! 1/4 =   0.25<br>
//! 1/5 =   0.2<br>
//! 1/6 =   0.1(6)<br>
//! 1/7 =   0.(142857)<br>
//! 1/8 =   0.125<br>
//! 1/9 =   0.(1)<br>
//! 1/10    =   0.1
//!
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has
//! a 6-digit recurring cycle.
//!
//! Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal
//! fraction part.
//!
//! # Solution detail
//!
//! The most interesting element is in how to find the cycle length of 1/d:
//!
//! The digits in the decimal expansion of the fraction `1/d` can be found by performing a long
//! division of `d` into the number `100000...`. This leads to the following way of calculating
//! the length of the cycle.
//!
//! 1. Remove all factors of 2 or 5 from `d`. This is because dividing by 2 or 5 is equivalent to
//! first multipying by 5 or 2 and subsequently dividing by 10, neither of which changes the cycle
//! length.
//!
//! 2. With the resulting number `d'`, find the order of 10 modulo `d'`. This is the length of the
//! period of `1/d'`, since it is when the remainders of dividing successive powers of 10 by `d'`
//! start to repeat in sequence. This exactly means that the digits in the computed decimal
//! expansion (which is, after all, found by long division) will repeat.
//!
//! We can further note that, since there are only `d - 1` possible remainders when dividing by
//! `d`, the length of the cycle of `1/d` is at most `d - 1`. If we start at large values of `d`
//! and work our way down, we may be able to terminate early when the later values of `d` will be
//! too small.
//!
//! The final optimisation is to only consider those values of `d` which are not divisible by 2 or
//! 5, since other values of `d` will be equivalent to smaller values of `d` which result from
//! dividing out the 2 and 5.

use itertools::Itertools;

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 26";
/// A description of the problem.
pub const DESC: &'static str = "Reciprocal cycles";

/// Find the length of the cycle of the decimal 1/d, where d is coprime to 10. Requires a prime
/// sieve capable of factorising numbers up to d (that is, containing primes up to the square root
/// of d).
fn cycle_length(d: u32, sieve: &Sieve) -> u64 {
    sieve.order(10, d).unwrap()
}

/// Find the number below the given limit with the longest repeating decimal.
fn solve(lim: u32) -> u32 {
    // Sieve for enough primes to calculate orders with respect to moduli as large as the limit.
    let sieve = Sieve::to_limit(integer_sqrt(lim as u64));

    // For numbers down from the limit, calculate the cycle lengths and find the longest one.
    let mut best_length = 0u64;
    let mut best_d = 0;
    for d in (1..lim).rev().skip((lim % 2) as usize).step(2) {
        // Break early if following values of d are too small to have a long enough cycle.
        if (d as u64) < best_length {
            break;
        } else if d % 5 != 0 {
            // Calculate the cycle length and check if it is the best so far.
            let length = cycle_length(d, &sieve);
            if length > best_length {
                best_length = length;
                best_d = d;
            }
        }
    }

    best_d
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1_000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem026() {
        assert_eq!(super::answer(), "983");
    }
}
