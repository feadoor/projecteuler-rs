//! [Problem 37 (Truncatable primes)](https://projecteuler.net/problem=37)
//!
//! # Problem statement
//!
//! The number 3797 has an interesting property. Being prime itself, it is possible to continuously
//! remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
//! Similarly we can work from right to left: 3797, 379, 37, and 3.
//!
//! Find the sum of the only eleven primes that are both truncatable from left to right and right
//! to left.
//!
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
//!
//! # Solution detail
//!
//! There is an easy method to generate all the right-truncatable primes: start with the single
//! digit primes 2, 3, 5 and 7 as roots, and do a depth-first traversal of the tree of numbers
//! formed by adding one of the digits 1, 3, 7, 9 to the end of the previous number. We are only
//! interested in the nodes which represent prime numbers, so we prune any branches leading to a
//! non-prime.
//!
//! Then, to solve the problem, simply iterate over the right-truncatable primes, and check if all
//! of them are also left-truncatable.
//!
//! Note that we have no a priori upper bound on how large these primes can get, so we will perform
//! primality testing using a prime sieve that starts at some small size and grows when necessary.

use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 37";
/// A description of the problem.
pub const DESC: &'static str = "Truncatable primes";

// A generic iterator over some u64s
type NumIter = Box<Iterator<Item = u64>>;

/// A function which tests for primality using the given Sieve, expanding the sieve if necessary.
fn is_prime_safe(n: u64, sieve: &mut Sieve) -> bool {

    // Expand the sieve to be large enough to test n for primality.
    while sieve.limit().saturating_mul(sieve.limit()) < n {
        *sieve = Sieve::to_limit(10 * sieve.limit());
    }

    sieve.is_prime(n).unwrap()
}

/// Check if the given number is a left-truncatable prime.
fn is_left_truncatable(n: u64, sieve: &mut Sieve) -> bool {

    let mut power = 1;
    while power < n {
        power *= 10;
        if !is_prime_safe(n % power, sieve) { return false; }
    }

    true
}

/// An iterator over all the right-truncatable primes. Requires a Sieve to perform primality
/// testing, although the exact size of the sieve is not important as it will be expanded as
/// necessary.
fn right_truncatables(sieve: &mut Sieve) -> NumIter {

    // An inner function which recursively calls itself to perform a depth-first traversal of the
    // right truncatable primes. Generates the right truncatable primes formed from the current
    // number by appending digits 1, 3, 7 and 9.
    fn generator(curr_num: u64, sieve: &mut Sieve) -> NumIter {
        let extensions = [1, 3, 7, 9].iter()
            .map(|d| 10 * curr_num + d)
            .filter_map(|x| if is_prime_safe(x, sieve) { Some(generator(x, sieve)) } else { None })
            .fold(Box::new(None.into_iter()) as NumIter,
                  |acc, x| Box::new(acc.chain(x)));

        Box::new(Some(curr_num).into_iter().chain(extensions))
    }

    [2, 3, 5, 7].iter()
        .map(|&x| generator(x, sieve))
        .fold(Box::new(None.into_iter()),
              |acc, x| Box::new(acc.chain(x)))
}

/// Find the sum of all primes which are both left-truncatable and right-truncatable. Remember to
/// subtract the single-digit primes which are not counted.
fn solve() -> u64 {
    let mut sieve = Sieve::to_limit(1000);
    right_truncatables(&mut sieve)
        .filter(|&x| is_left_truncatable(x, &mut sieve)).sum::<u64>() - 17
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem037() {
        assert_eq!(super::answer(), "748317");
    }
}
