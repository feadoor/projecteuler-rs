//! [Problem 77 (Prime summations)](https://projecteuler.net/problem=77)
//!
//! # Solution detail
//!
//! An iterative dynamic programming approach is best here.
//!
//! First, we need an upper bound on the answer, so we know how many primes to consider. An
//! extremely crude upper bound is 3n - we can write this as a sum of 2's and 3's in at least n
//! ways. So to guarantee to find the answer, sieving all primes up to 3n will suffice. To actually
//! find the answer:
//!
//! Start with an array indexed by 0, 1, 2, ..., 3n, with `a[0]` = 1 and all other `a[i]` = 0. Note
//! that initially, `a` contains the numbers of ways of producing each target 0, 1, 2, ..., 3n using
//! primes taking values from the empty set.
//!
//! We will iteratively add one prime at a time to the pool of possible primes, updating the array
//! as we do so. Indeed, suppose we add a prime with value p. Then we update `a` as follows:
//!
//! ```pseudocode
//! for j = p to end of array:
//!     a[j] += a[j-p]
//! ```
//!
//! After doing this for a prime p, we can chop the array off after any index with an entry at least
//! n, because we know the answer will not exceed that index.
//!
//! After doing this process for all primes p below our upper bound, the first index with an entry
//! over n is the answer.

use primesieve::Sieve;
use projecteuler_rs::problem;

/// Finds the first number that can be written as a sum of primes in at least the given number of
/// ways.
fn solve(required_ways: usize) -> usize {

    // Create the array `a` which will eventually hold the final answer.
    let upper_bound = 3 * required_ways;
    let mut a = vec![0; upper_bound + 1];
    a[0] = 1;

    // For each prime, update the array so that `a[i]` holds the number of ways of making
    // `i` using the coins considered so far.
    for p in Sieve::to_limit(upper_bound as u64).iter() {
        for j in (p as usize)..a.len() {
            a[j] += a[j - (p as usize)];

            // Chop the array if we can improve our upper bound
            if a[j] >= required_ways {
                a.truncate(j + 1);
                break;
            }
        }
    }

    // Return the answer, which at this point is just the index of the last element of the array
    a.len() - 1
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(5_000).to_string()
}

problem!(answer, "71");
