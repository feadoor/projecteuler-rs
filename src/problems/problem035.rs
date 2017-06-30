//! [Problem 35 (Circular primes)](https://projecteuler.net/problem=35)
//!
//! # Problem statement
//!
//! The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and
//! 719, are themselves prime.
//!
//! There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//!
//! How many circular primes are there below one million?
//!
//! # Solution detail
//!
//! First, note that circular primes, other than the single digit primes 2 and 5, must consist
//! entirely of the digits 1, 3, 7 and 9, since all other primes end in these digits.
//!
//! This means that we can simply consider all combinations of these digits, and for each one,
//! check if the corresponding number is a circular prime. We do this prime checking using a sieve
//! which holds enough pre-computed prime numbers to do primality checking for each number we might
//! encounter.

use primesieve::Sieve;
use number_theory::{integer_sqrt, pow};

/// The name of the problem.
pub const NAME: &'static str = "Problem 35";
/// A description of the problem.
pub const DESC: &'static str = "Circular primes";

// A custom type which is a generic iterator over some u64s
type NumIter = Box<Iterator<Item = u64>>;

/// Check if the given number is a circular prime.
fn is_circular_prime(mut n: u64, sieve: &Sieve) -> bool {

    // Deal with a trivial case
    if n == 0 { return false; }

    // First, find the length of `n` so that digit rotations can be performed easily.
    let num_digits = 1 + (n as f64).log(10.0).floor() as u64;
    let power_of_ten = pow(10, num_digits - 1);

    // Now check that each rotation is prime.
    for _ in 0..num_digits {
        if !sieve.is_prime(n).unwrap() {
            return false;
        } else {
            let last_digit = n % 10;
            n = n / 10 + last_digit * power_of_ten;
        }
    }

    true
}

/// Construct all numbers consisting only of the digits 1, 3, 7 and 9, not exceeding the given
/// number of digits.
fn numbers_to_check(digits: u64) -> NumIter {

    // Define an inner recursive function which finds the numbers we need using a depth-first
    // algorithm, building them up one digit at a time.
    fn generator(curr_num: u64, digits_left: u64) -> NumIter {
        // If we have some spare digits, then chain together all choices coming from each possible
        // choice of next digit. Otherwise, just produce the current number,
        if digits_left == 0 {
            Box::new(Some(curr_num).into_iter())
        } else {
            let extensions = [1, 3, 7, 9]
                .iter()
                .fold(Box::new(None.into_iter()) as NumIter,
                      |acc, x| Box::new(acc.chain(generator(curr_num * 10 + x, digits_left - 1))));
            Box::new(extensions.chain(Box::new(Some(curr_num).into_iter())))

        }
    }

    generator(0, digits)
}

/// Find the number of circular primes there are with at most the given number of digits. Remember
/// to add on 2 for the primes 2 and 5 which are not considered otherwise.
fn solve(digits: u64) -> usize {
    let sieve = Sieve::to_limit(integer_sqrt(pow(10, digits)));
    numbers_to_check(digits).filter(|&x| is_circular_prime(x, &sieve)).count() + 2
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(6).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem035() {
        assert_eq!(super::answer(), "55");
    }
}
