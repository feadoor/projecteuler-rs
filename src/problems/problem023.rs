//! [Problem 23 (Non-abundant sums)](https://projecteuler.net/problem=23)
//!
//! # Problem statement
//!
//! A perfect number is a number for which the sum of its proper divisors is exactly equal to the
//! number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
//! which means that 28 is a perfect number.
//!
//! A number n is called deficient if the sum of its proper divisors is less than n and it is
//! called abundant if this sum exceeds n.
//!
//! As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
//! written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown
//! that all integers greater than 28123 can be written as the sum of two abundant numbers.
//! However, this upper limit cannot be reduced any further by analysis even though it is known
//! that the greatest number that cannot be expressed as the sum of two abundant numbers is less
//! than this limit.
//!
//! Find the sum of all the positive integers which cannot be written as the sum of two abundant
//! numbers.
//!
//! # Solution detail
//!
//! See [here](http://mathschallenge.net/full/sum_of_two_abundant_numbers) for a proof that all
//! numbers greater than 28123 are the sum of two abundant numbers.
//!
//! To find the numbers which are not the sum of two abundant numbers, therefore, it will be
//! enough to check every number up to 28123, and store whether each number is abundant in a
//! boolean array. Then, to see if a given number `n` is a sum of two abundant numbers, it will be
//! enough to iterate over the abundant numbers `x` and check if `n - x` is abundant.
//!
//! The abundant numbers themselves can be found by brute force, using prime factorisation to
//! calculate the sum of divisors.

use number_theory::integer_sqrt;
use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 23";
/// A description of the problem.
pub const DESC: &'static str = "Non-abundant sums";

/// Check whether n is amicable.
fn is_sum_of_abundants(n: usize, abundants: &[bool]) -> bool {
    for abundant in 1..abundants.len() {
        if n < abundant + abundant {
            break;
        } else if abundants[abundant] && abundants[n - abundant] {
            return true;
        }
    }

    false
}

/// Find the abundant numbers up to the given limit.
fn abundants(lim: u64) -> Vec<bool> {
    // First, sieve out all the prime numbers needed to factorise numbers up to the limit.
    let sieve = Sieve::to_limit(integer_sqrt(lim));

    // Now factorise each number and store the abundant ones.
    (0..lim).map(|x| x != 0 && sieve.sum_of_divisors(x).unwrap() > x + x).collect()
}

/// Find the sum of the numbers below the given limit which are not a sum of two amicable numbers.
fn solve(lim: u64) -> u64 {
    let abundants = abundants(lim);
    (1..lim as usize).filter(|&x| !is_sum_of_abundants(x, &abundants)).sum::<usize>() as u64
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(28123).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem023() {
        assert_eq!(super::answer(), "4179871");
    }
}
