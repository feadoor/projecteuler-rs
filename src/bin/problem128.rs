//! [Problem 128 (Hexagonal tile differences)](https://projecteuler.net/problem=128)
//!
//! # Solution detail
//!
//! The trick with this problem is to eliminate large swathes of tiles without needing to do any
//! calculation. Apart from the first two layers, which are close enough to the special case
//! 2 (being the only even prime), only the very first and very last tiles in any given layer have
//! any chance of having three prime differences. That is because every other tile has two immediate
//! neighbours as adjacent tiles, and of the other four tiles, two are odd and two are even, so that
//! at most two of the differences can be prime.
//!
//! Crunching some numbers tells us that the first and last tiles in each layer are given by the
//! following formulae:
//!
//! FIRST(n + 2) = 3n² + 3n + 2,  LAST(n + 2) = 3n² + 9n + 7
//!
//! The differences around the first tile are:
//!
//! 1, 6n, 6(n+1), 6(n+1) - 1, 6(n+1) + 1, 12(n+1) + 5
//!
//! And the differences around the last tile are:
//!
//! 1, 6(n+1), 6(n+2), 6(n+1) - 1, 6n + 11, 12n + 5
//!
//! So we just need to check these numbers for primality when working out which tiles have three
//! prime differences. This can be done using, for example, the deterministic form of the
//! Miller-Rabin primality test.

use number_theory::is_prime;
use projecteuler_rs::problem;

/// Find the kth tile with 3 prime differences.
fn solve(k: u64) -> u64 {

    // Already count tiles 1 and 2 form the first two layers.
    let mut count = 2;

    // Iterate over all the layers until we've found enough tiles.
    for n in 1.. {

        // Check the first tile in the layer
        if is_prime(6 * n + 5) && is_prime(6 * n + 7) && is_prime(12 * n + 17) {
            count += 1;
            if count == k { return 3 * n * (n + 1) + 2; }
        }

        // Check the last tile in the layer
        if is_prime(6 * n + 5) && is_prime(6 * n + 11) && is_prime(12 * n + 5) {
            count += 1;
            if count == k { return 3 * (n + 1) * (n + 2) + 1; }
        }
    }

    unreachable!()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2_000).to_string()
}

problem!(answer, "14516824220");
