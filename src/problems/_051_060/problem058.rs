//! [Problem 58 (Spiral primes)](https://projecteuler.net/problem=58)
//!
//! # Problem statement
//!
//! Starting with 1 and spiralling anticlockwise in the following way, a square spiral with side
//! length 7 is formed.
//!
//!     37 36 35 34 33 32 31
//!     38 17 16 15 14 13 30
//!     39 18  5  4  3 12 29
//!     40 19  6  1  2 11 28
//!     41 20  7  8  9 10 27
//!     42 21 22 23 24 25 26
//!     43 44 45 46 47 48 49
//!
//! It is interesting to note that the odd squares lie along the bottom right diagonal, but what is
//! more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is,
//! a ratio of 8/13 ≈ 62%.
//!
//! If one complete new layer is wrapped around the spiral above, a square spiral with side length
//! 9 will be formed. If this process is continued, what is the side length of the square spiral for
//! which the ratio of primes along both diagonals first falls below 10%?
//!
//! # Solution detail
//!
//! The numbers along each diagonal are generated by a particular quadratic expression. These
//! expressions are, for n = 0, 1, 2...
//!
//! - **Bottom-right**: (2n + 1)<sup>2</sup>
//! - **Bottom-left**: (2n + 1)<sup>2</sup> - 2n
//! - **Top-left**: (2n + 1)<sup>2</sup> - 4n
//! - **Top-right**: (2n + 1)<sup>2</sup> - 6n
//!
//! So we can simply generate the terms in these quadratic sequences, check them for primality, and
//! keep track of the ratio of prime terms as we go. To do primality checks, it is convenient to
//! have a precomputed prime sieve. If it turns out that the sieve is not large enough, then we can
//! increase the size as we go.

use primesieve::Sieve;

/// The name of the problem.
pub const NAME: &'static str = "Problem 58";
/// A description of the problem.
pub const DESC: &'static str = "Spiral primes";

/// Check if the given number is prime using the given prime sieve, expanding the size of the prime
/// sieve if necessary.
fn is_prime_safe(n: u64, sieve: &mut Sieve) -> bool {
    loop {
        match sieve.is_prime(n) {
            Ok(x) => return x,
            Err(_) => *sieve = Sieve::to_limit(2 * sieve.limit()),
        }
    }
}

/// Find how many layers of a spiral grid must be generated before the ratio of primes on the
/// diagonals falls below 10% for the first time.
fn solve() -> u64 {
    // Create a sieve to use for primality testing.
    let mut sieve = Sieve::to_limit(10_000);

    // Keep track of the size of the spiral grid, the number of terms on the diagonals, and the
    // amount of these terms which are primes. Start with a grid of size 3.
    let (mut size, mut terms, mut primes) = (2, 5, 3);
    while terms <= 10 * primes {

        // Calculate the number of new terms which are prime.
        let br_term = 4 * size * (size + 1) + 1;
        for &term in &[br_term, br_term - 2 * size, br_term - 4 * size, br_term - 6 * size] {
            if is_prime_safe(term, &mut sieve) {
                primes += 1;
            }
        }

        // Update the size and the overall number of terms.
        size += 1;
        terms += 4;
    }

    2 * size - 1
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem058() {
        assert_eq!(super::answer(), "26241");
    }
}
