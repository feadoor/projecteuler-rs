//! [Problem 62 (Cubic permutations)](https://projecteuler.net/problem=62)
//!
//! # Solution detail
//!
//! This problem can be solved using the following algorithm:
//!
//!  - For each range 1...9, 10...99, 100...999, iterate over the cubes in that range.
//!
//!  - Create a hash table, with keys given by sets of digits, and values given by the number of
//!  cubes having those digits.
//!
//!  - After getting to the end of a range, check for keys in the hash table with five cubes, and
//!  find the smallest cube corresponding to such an entry.
//!
//! To speed up the last of these steps, we can store in the hash table not only the number of cubes
//! having a given set of digits, but also the smallest cube having those digits.

#[macro_use]
extern crate projecteuler_rs;

use std::collections::HashMap;

/// Get the digits of the given number, in sorted order.
fn get_sorted_digits(mut n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.sort();

    digits
}

/// Find the smallest cube having exactly the given number of permutations of
/// its digits which are themselves cubes.
fn solve(number_of_permutations: usize) -> u64 {
    let mut n = 1;
    let mut cube = 1;
    let mut range_limit = 10;

    loop {
        let mut digits_to_cubes = HashMap::new();
        while cube < range_limit {
            let digits = get_sorted_digits(cube);
            digits_to_cubes.entry(digits).or_insert((0, cube)).0 += 1;

            n += 1;
            cube = n * n * n;
        }

        if let Some(solution) = digits_to_cubes.values()
            .filter_map(|&v| if v.0 == number_of_permutations { Some(v.1) } else { None })
            .min() {
            return solution;
        }

        range_limit *= 10;
    }
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(5).to_string()
}

problem!(answer, "127035954683");
