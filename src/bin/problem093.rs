//! [Problem 93 (Arithmetic expressions)](https://projecteuler.net/problem=93)
//!
//! # Solution detail
//!
//! Brute-force is the easiest way to approach this problem - the question is how to implement it
//! efficiently and ergonomically!
//!
//! One approach is to consider what targets can be reached from particular sets of digits. Given
//! a particular set of digits, all possible targets are obtained by considering all the ways of
//! splitting that set into two non-empty subsets, taking all possible targets from the two subsets
//! and combining them with every possible operation.
//!
//! This framing of the problem leads to an iterative solution whereby the base case consists of
//! the singleton sets containing a single digit - for them, the only possible target is the number
//! itself.
//!
//! From there, iterate over the possible sets of digits in order of increasing size, combining
//! the answers to previous sets in all possible combinations.
//!
//! Once all targets have been calculated, it's a simple matter of filtering to integer answers and
//! doing a bit of counting.

use itertools::Itertools;
use num::traits::identities::Zero;
use num::rational::Ratio;
use projecteuler_rs::problem;
use std::collections::{HashMap, HashSet};

type DigitSet = (usize, usize, usize, usize);
type Target = Ratio<isize>;

/// A handy conversion to represent four digits as one number
fn digit_set_to_integer(ds: DigitSet) -> usize {
    ds.0 | (ds.1 << 4) | (ds.2 << 8) | (ds.3 << 12)
}

/// All results of a binary operation on the two given numbers
fn all_ops(t1: &Target, t2: &Target) -> Vec<Target> {
    let mut results = vec![t1 + t2, t1 - t2, t2 - t1, t1 * t2];
    if !t1.is_zero() { results.push(t2 / t1); }
    if !t2.is_zero() { results.push(t1 / t2); }
    results
}

/// Update the target `HashSet` with all possible operations on two numbers from the two given
/// `HashSet`s.
fn update(target: &mut HashSet<Target>, input1: &HashSet<Target>, input2: &HashSet<Target>) {
    for target1 in input1.iter() {
        for target2 in input2.iter() {
            for result in all_ops(target1, target2) {
                target.insert(result);
            }
        }
    }
}

/// Count the consecutive integers 1, 2, ..., n that exist in the given set of targets
fn count_consecutive(targets: &HashSet<Target>) -> usize {
    (1..).take_while(|&x| targets.contains(&Target::from_integer(x))).last().unwrap_or(0) as usize
}

/// Find the four digits which allow the most consecutive integer targets to be reached.
fn solve() -> DigitSet {
    let mut target_sets: HashMap<usize, HashSet<Target>> = HashMap::new();

    // Singleton digits
    for digit in 1..10 {
        let key = digit_set_to_integer((digit, 0, 0, 0));
        target_sets.entry(key).or_insert(HashSet::new()).insert(Target::from_integer(digit as isize));
    }

    // Combine singletons to make pairs
    for (digit1, digit2) in (1..10).tuple_combinations() {

        let mut target_set = HashSet::new();

        let key1 = digit_set_to_integer((digit1, 0, 0, 0));
        let key2 = digit_set_to_integer((digit2, 0, 0, 0));
        update(&mut target_set, &target_sets.get(&key1).unwrap(), &target_sets.get(&key2).unwrap());

        let key = digit_set_to_integer((digit1, digit2, 0, 0));
        target_sets.insert(key, target_set);
    }

    // Combine singletons and pairs to make triples
    for (d1, d2, d3) in (1..10).tuple_combinations() {

        let mut target_set = HashSet::new();

        for (digit1, digit2, digit3) in vec![(d1, d2, d3), (d2, d1, d3), (d3, d1, d2)] {
            let key1 = digit_set_to_integer((digit1, 0, 0, 0));
            let key2 = digit_set_to_integer((digit2, digit3, 0, 0));
            update(&mut target_set, &target_sets.get(&key1).unwrap(), &target_sets.get(&key2).unwrap());
        }

        let key = digit_set_to_integer((d1, d2, d3, 0));
        target_sets.insert(key, target_set);
    }

    // Combine singletons, pairs and triples to make quadruples
    for (d1, d2, d3, d4) in (1..10).tuple_combinations() {

        let mut target_set = HashSet::new();

        // Singleton + Triple
        for (digit1, digit2, digit3, digit4) in vec![(d1, d2, d3, d4), (d2, d1, d3, d4), (d3, d1, d2, d4), (d4, d1, d2, d3)] {
            let key1 = digit_set_to_integer((digit1, 0, 0, 0));
            let key2 = digit_set_to_integer((digit2, digit3, digit4, 0));
            update(&mut target_set, &target_sets.get(&key1).unwrap(), &target_sets.get(&key2).unwrap());
        }

        // Pair + Pair
        for (digit1, digit2, digit3, digit4) in vec![(d1, d2, d3, d4), (d1, d3, d2, d4), (d1, d4, d2, d3), (d2, d3, d1, d4), (d2, d4, d1, d3), (d3, d4, d1, d2)] {
            let key1 = digit_set_to_integer((digit1, digit2, 0, 0));
            let key2 = digit_set_to_integer((digit3, digit4, 0, 0));
            update(&mut target_set, &target_sets.get(&key1).unwrap(), &target_sets.get(&key2).unwrap());
        }

        let key = digit_set_to_integer((d1, d2, d3, d4));
        target_sets.insert(key, target_set);
    }

    // Loop through all quadruples and find the one with the best set of targets
    let mut best_quad = (0, 0, 0, 0); let mut best_count = 0;
    for (d1, d2, d3, d4) in (1..10).tuple_combinations() {
        let key = digit_set_to_integer((d1, d2, d3, d4));
        let target_set = target_sets.entry(key).or_insert(HashSet::new());
        let count = count_consecutive(&target_set);
        if count > best_count { best_count = count; best_quad = (d1, d2, d3, d4); }
    }

    best_quad
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let best_quad = solve();
    format!("{}{}{}{}", best_quad.0, best_quad.1, best_quad.2, best_quad.3)
}

problem!(answer, "1258");
