//! [Problem 42 (Coded triangle numbers)](https://projecteuler.net/problem=42)
//!
//! # Problem statement
//!
//! The nth term of the sequence of triangle numbers is given by, t<sub>n</sub> = n(n+1)/2; so the
//! first ten triangle numbers are:
//!
//! 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//!
//! By converting each letter in a word to a number corresponding to its alphabetical position and
//! adding these values we form a word value. For example, the word value for SKY is
//! 19 + 11 + 25 = 55 = t<sub>10</sub>. If the word value is a triangle number then we shall call
//! the word a triangle word.
//!
//! Using [words.txt](https://projecteuler.net/project/resources/p042_words.txt), a 16K text file
//! containing nearly two-thousand common English words, how many are triangle words?
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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::from_utf8;

use number_theory::is_square;

/// The name of the problem.
pub const NAME: &'static str = "Problem 42";
/// A description of the problem.
pub const DESC: &'static str = "Coded triangle numbers";

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
pub fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem042.txt")).unwrap();
    let reader = BufReader::new(file);
    let names = reader.split(b',')
        .map(|x| x.unwrap())
        .map(|x| from_utf8(&x).unwrap().trim_matches('"').to_string());
    solve(names).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem042() {
        assert_eq!(super::answer(), "162");
    }
}
