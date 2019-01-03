//! [Problem 90 (Cube digit pairs)](https://projecteuler.net/problem=90)
//!
//! # Solution detail
//!
//! This is another problem where it's difficult to improve on a well-written brute-force solution.
//!
//! Simply iterate over all pairs of combinations of faces for the two dice, remember that a 6 and
//! a 9 are interchangeable, and check if all of the two-digit squares can be formed.

use projecteuler_rs::problem;

use itertools::Itertools;
use std::collections::HashSet;

type Die = HashSet<usize>;

/// All of the two-digit squares, given as a pair of digits.
const SQUARES: &'static [(usize, usize); 9] = &[
    (0, 1), (0, 4), (0, 9), (1, 6), (2, 5), (3, 6), (4, 9), (6, 4), (8, 1),
];

/// Check if the given dice are capaable of representing all the squares.
fn can_represent_squares(die1: &Die, die2: &Die) -> bool {
    SQUARES.iter().all(|&(a, b)|
        (die1.contains(&a) && die2.contains(&b)) || (die1.contains(&b) && die2.contains(&a))
    )
}

fn make_die(numbers: Vec<usize>) -> Die {
    let mut die: Die = numbers.iter().map(|x| *x).collect();
    if die.contains(&6) { die.insert(9); }
    if die.contains(&9) { die.insert(6); }
    die
}

/// Find the number of dice arrangements allowing the two-digit squares to be formed.
fn solve() -> usize {
    let dice = (0..10).combinations(6).map(make_die);
    dice.combinations(2)
        .filter(|dice| can_represent_squares(&dice[0], &dice[1]))
        .count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "1217");
