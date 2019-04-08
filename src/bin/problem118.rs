//! [Problem 118 (Pandigital prime sets)](https://projecteuler.net/problem=118)
//!
//! # Solution detail
//!
//! Because of the unstructured way in which primes are distributed, any solution will need some
//! element of brute-force. The key is in minimising the amount of repeated work.
//!
//! One approach is to first, for each subset of the digits 1..9, check all permutations of those
//! digits and work out how many of them are prime. Store that count in a `HashMap` keyed by the
//! set of digits.
//!
//! Then iterate over all ways of partitioning 1..9 into distinct subsets, and for each partition,
//! calculate how many ways the digits can be arranged so that all items in the partition are prime.
//! This is just the product of the number of permutations of each part that are prime, calculated
//! earlier and stored in the `HashMap`.

use std::collections::HashMap;

use combinatorics::{all_subsets, all_partitions, Permutations};
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Turn the given digits into a number
fn to_integer(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |acc, d| 10 * acc + d)
}

/// Find the number of permutations of the given digits which are prime, using the given sieve for
/// primality testing.
fn prime_permutations(digits: &[u64], sieve: &Sieve) -> usize {
    digits.iter().map(|x| *x).permutations()
        .map(|perm| to_integer(&perm))
        .filter(|&n| sieve.is_prime(n).unwrap())
        .count()
}

/// Find the number of pandigital prime sets.
fn solve() -> usize {

    // Somewhere to store the counts, for each set of digits, how many primes they make.
    let mut prime_perms = HashMap::new();
    let sieve = Sieve::to_limit(100_000);

    // Iterate over all subsets of the digits and stores the prime counts
    for mut subset in all_subsets((1..10).collect()) {
        subset.sort();
        let prime_perms_count = prime_permutations(&subset, &sieve);
        prime_perms.insert(subset, prime_perms_count);
    }

    // For each partition, count the number of ways of arranging the digits into a prime and sum.
    all_partitions((1..10).collect()).map(|partition| 
        partition.iter().map(|part| prime_perms.get(part).unwrap()).product::<usize>()
    ).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "44680");
