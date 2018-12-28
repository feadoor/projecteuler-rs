//! [Problem 30 (Digit fifth powers)](https://projecteuler.net/problem=30)
//!
//! # Solution detail
//!
//! We can find the solutions to this problem using a backtracking-style algorithm, building up
//! the solutions from left-to-right one digit at a time. Note that since 7 × 9<sup>5</sup> has
//! only 6 digits, there cannot be any solutions with more than 6 digits. This means that our
//! search tree only has depth 6.
//!
//! We can also prune the search tree before reaching the bottom by calculating the maximum and
//! minimum values that can be obtained by by the sum of fifth powers after extending the number
//! to length 6, and seeing if the number itself can ever fall within that range.
//!
//! For example, no matter what three digits we add to the end of 211, the sum of fifth powers can
//! never be larger than 2<sup>5</sup> + 1 + 1 + 9<sup>5</sup> + 9<sup>5</sup> + 9<sup>5</sup>,
//! which is equal to 177181. On the other hand, the number itself after adding three digits will
//! always be at least 211000, so there cannot possibly be a solution.
//!
//! Finally, remember to subtract 1 at the end, since it is not counted as a solution.

#![feature(generators, generator_trait)]

use generators::{GeneratorIteratorAdapter, yield_from};
use projecteuler_rs::problem;
use std::ops::{Generator, GeneratorState};

/// Fifth powers of single digits.
const FIFTH_POWER: &'static [u64; 10] = &[0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

/// Find all numbers which are equal to the sum of the fifth powers of their digits, using
/// a depth-first search on the digits of the solution.
fn fifth_power_sum_numbers(max_length: u64) -> impl Iterator<Item = u64> {

    #[derive(Clone, Copy)]
    struct SearchEnv   { max_length: u64, }
    struct SearchState { value: u64, power_sum: u64, length: u64, }

    /// Check if the current search state can possibly be extended to a solution, by examining the
    /// maximum and minimum values that can be taken by the power sum from this point.
    fn may_be_extended(state: &SearchState, env: &SearchEnv) -> bool {

        // Calculate the maximum and minimum values taken by the fifth power sum.
        let min_power_sum = state.power_sum;
        let max_power_sum = state.power_sum + (env.max_length - state.length) * FIFTH_POWER[9];

        // Calculate the maximum and minimum values taken by the actual number.
        let mut min_value = state.value;
        let mut max_value = state.value + 1;
        for _ in 0.. env.max_length - state.length { min_value *= 10; max_value *= 10; }

        // Check if the two ranges overlap, otherwise there is definitely no solution.
        min_power_sum < max_value && max_power_sum >= min_value
    }

    /// The state that arises from the current state by appending a single digit
    fn new_state(state: &SearchState, digit: u64) -> SearchState {
        SearchState {
            value: 10 * state.value + digit,
            power_sum: state.power_sum + FIFTH_POWER[digit as usize],
            length: state.length + 1,
        }
    }

    /// An inner function that facilitates the search by recursively calling itself to add
    /// digits to the number.
    fn solutions_beneath(state: SearchState, env: SearchEnv) -> Box<dyn Generator<Yield=u64, Return=()>> {
        Box::new(move || {
            if state.length == env.max_length && state.value == state.power_sum { yield state.value; }
            if state.length < env.max_length && may_be_extended(&state, &env) {
                for digit in 0..10 {
                    yield_from!(solutions_beneath(new_state(&state, digit), env));
                }
            }
        })
    }

    let initial_state = SearchState { value: 0, power_sum: 0, length: 0};
    let search_environment = SearchEnv { max_length: max_length };
    GeneratorIteratorAdapter::of(solutions_beneath(initial_state, search_environment))
}

/// Find the sum of the numbers which are equal to the sum of the fifth powers of their digits.
fn solve() -> u64 {
    fifth_power_sum_numbers(6).sum::<u64>() - 1
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "443839");
