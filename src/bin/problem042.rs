//! [Problem 42 (Coded triangle numbers)](https://projecteuler.net/problem=42)
//!
//! # Solution detail
//!
//! What we need is an efficient method of deciding whether a given number is triangular. The
//! defining equation m = n(n+1)/2 rearranges to give:
///
/// `n = (sqrt(8m + 1) - 1) / 2`
///
/// This means that a given number m is triangular exactly when 8m + 1 is a square.
///
/// Now that we have a method for deciding if a number is triangular, all that's left is to go
/// through the names, convert them to numbers and check each one.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;
use number_theory::is_square;

/// Decide if a given number is triangular.
fn is_triangular(n: u64) -> bool {
    is_square(8 * n + 1)
}

/// Find how many of the given words are triangle words.
fn solve<I>(words: I) -> usize
    where I: Iterator<Item = String>
{
    let score = |word: &str| -> u64 { word.as_bytes().iter().map(|x| (x - b'A' + 1) as u64).sum() };

    words.map(|x| score(&x)).filter(|&x| is_triangular(x)).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem042.txt")).unwrap();
    let reader = BufReader::new(file);
    let names = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim_matches('"').to_string());
    solve(names).to_string()
}

problem!(answer, "162");
