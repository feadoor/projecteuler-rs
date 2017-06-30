//! [Problem 49 (Prime permutations)](https://projecteuler.net/problem=49)
//!
//! # Problem statement
//!
//! The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is
//! unusual in two ways:
//!
//! (i) each of the three terms are prime, and,
//!
//! (ii) each of the 4-digit numbers are permutations of one another.
//!
//! There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
//! property, but there is one other 4-digit increasing sequence.
//!
//! What 12-digit number do you form by concatenating the three terms in this sequence?
//!
//! # Solution detail
//!
//! There's nothing much cleverer than brute-force here. Generate all the 4-digit primes, and then
//! for each prime, generate all of its permutations which are prime, and check for an aritmetic
//! progression.

use permutohedron::Heap;
use primesieve::Sieve;

use std::collections::HashSet;
use std::iter::Iterator;

/// The name of the problem.
pub const NAME: &'static str = "Problem 49";
/// A description of the problem.
pub const DESC: &'static str = "Prime permutations";

/// Convert a vector of digits into an actual number.
fn to_int(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |acc, x| 10 * acc + x)
}

/// Convert a number into a vector of digits.
fn to_digits(mut n: u64) -> Vec<u64> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

/// Finds the permutations of n which belong to the given set.
fn permutations_in_set(n: u64, set: &HashSet<u64>) -> HashSet<u64> {
    let mut digits = to_digits(n);
    Heap::new(&mut digits).map(|x| to_int(&x)).filter(|x| set.contains(x)).collect()
}

/// Finds the arithmetic progression of three 4-digit primes, other than 1487, 4817, 8147, for which
/// the terms are permutations of one another.
fn solve() -> Vec<u64> {
    let mut answers = Vec::new();
    let ignored = (1487, 4817, 8147);

    // Iterate over primes
    let primes: HashSet<_> = Sieve::to_limit(10_000).iter().collect();
    for &prime in primes.iter() {

        // Check for permutations of this prime which are themselves prime
        let permuted_primes = permutations_in_set(prime, &primes);
        for &permuted_prime in permuted_primes.iter().filter(|&&perm| perm > prime) {

            // Check if the number which completes the progression is also a prime permutation
            let other = permuted_prime + (permuted_prime - prime);
            if permuted_primes.contains(&other) && (prime, permuted_prime, other) != ignored {
                answers.push(vec![prime, permuted_prime, other]);
            }
        }
    }

    assert!(answers.len() == 1);
    answers.remove(0)
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().iter().fold(String::new(), |x, y| x + &y.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem049() {
        assert_eq!(super::answer(), "296962999629");
    }
}
