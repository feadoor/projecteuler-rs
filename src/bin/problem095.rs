//! [Problem 95 (Amicable chains)](https://projecteuler.net/problem=95)
//!
//! # Solution detail
//!
//! This is another problem that brute-force is good enough for, assuming that you are computing
//! the sum of divisors efficiently using prime factorisation.
//!
//! Start by computing the sum of divisors of every number below the limit, and store them in an
//! array (or store Option::None if the sum of divisors exceeds the limit).
//!
//! Once the array has been pre-computed, just check every single number to see if it is part of a
//! cycle, and keep track of the longest cycle seen and the smallest number in it.

use number_theory::integer_sqrt;
use primesieve::Sieve;
use projecteuler_rs::problem;

/// Find the cycle length of each starting number under the process n --> arr[n].
fn find_cycle_lengths(arr: &[Option<u64>]) -> Vec<Option<usize>> {

    // Somewhere to store the eventual lengths
    let mut cycle_lengths = vec![None; arr.len()];

    // Iterate over each starting item
    for mut chain_item in 0..arr.len() {

        // If we already know the length, do nothing
        if cycle_lengths[chain_item].is_none() {

            // Keep track of the items already seen in the process, for cycle detection
            let mut seen = Vec::new();
            loop {

                // If we've seen this item before, then save the cycle length of all seen items
                if let Some(idx) = seen.iter().position(|&x| x == chain_item) {
                    let cycle_length = seen.len() - idx;
                    for &tail_item in &seen[0..idx] { cycle_lengths[tail_item] = Some(0); }
                    for &cycle_item in &seen[idx..] { cycle_lengths[cycle_item] = Some(cycle_length); }
                    break;
                }

                // Otherwise update the current value and move to the next iteration
                else if let Some(next_item) = arr[chain_item] {
                    seen.push(chain_item);
                    chain_item = next_item as usize;
                } else {
                    break;
                }
            }
        }
    }

    cycle_lengths
}

/// Find the smallest member of the longest amicable chain with all members below the given limit.
fn solve(limit: usize) -> usize {

    // Calculate the sum of the divisors of each number
    let sieve = Sieve::to_limit(integer_sqrt(limit as u64));
    let sums_of_divisors: Vec<_> = (0..limit as u64 + 1)
        .map(|n| if n == 0 { 0 } else { sieve.sum_of_divisors(n).unwrap() - n })
        .map(|s| match s {
            0 => None,
            s if s <= limit as u64 => Some(s),
            _ => None,
        }).collect();

    // Find the cycle length of each starting number and find the number with the longest cycle
    let cycle_lengths = find_cycle_lengths(&sums_of_divisors);
    (1..limit + 1).rev().max_by_key(|&n| cycle_lengths[n]).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000).to_string()
}

problem!(answer, "14316");
