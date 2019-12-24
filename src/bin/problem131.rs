//! [Problem 131 (Prime cube partnership)](https://projecteuler.net/problem=131)
//!
//! # Solution detail
//!
//! Since `p` is prime and n²(n + p) is a perfect cube, both of its factors (which are coprime)
//! must be themselves cubes - that is, n and n + p must both be perfect cubes.
//!
//! Then these are two cubes whose difference is `p`, a prime. Since the difference of a³ and b³ is
//! itself divisible by a - b, in order for the difference to be prime it must be a difference of
//! two consecutive cubes.
//!
//! That means we can simply check the differences of consecutive cubes, and record how many are
//! prime.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the number of primes below the given limit with the cube property.
fn solve(limit: u64) -> u64 {
    let sieve = Sieve::to_limit(limit);
    let (mut n, mut cube_difference, mut ans) = (1, 7, 0);
    while cube_difference < limit {
        if sieve.is_prime(cube_difference).unwrap() { ans += 1; }
        n += 1;
        cube_difference += 6 * n;
    }
    ans
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "173");
