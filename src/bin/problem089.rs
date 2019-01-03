//! [Problem 89 (Roman numerals)](https://projecteuler.net/problem=89)
//!
//! # Solution detail
//!
//! No clever tricks here - for each line in the file, just read the Roman numerals to translate it
//! to a number, and then rewrite that number using minimal numerals and count the characters saved
//! in each case.

use projecteuler_rs::problem;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn numeral_value(single_numeral: char) -> usize {
    match single_numeral {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _   => unreachable!(),
    }
}

/// Read the given numeral as an integer.
fn read_numeral(numeral: &str) -> usize {
    let characters: Vec<_> = numeral.chars().collect(); let mut result = 0;
    for (idx, &chr) in characters.iter().enumerate() {
        if idx + 1 < characters.len() && numeral_value(chr) < numeral_value(characters[idx + 1]) {
            result -= numeral_value(chr);
        } else {
            result += numeral_value(chr);
        }
    }
    result
}

/// Write the given integer as a Roman numeral.
fn write_numeral(mut num: usize) -> String {
    let mut result: String = String::new();

    if num >= 1000 { result += &"M".repeat(num / 1000); num %= 1000; }
    if num >= 900  { result += "CM"; num -= 900; }
    if num >= 500  { result += "D"; num -= 500; }
    if num >= 400  { result += "CD"; num -= 400; }
    if num >= 100  { result += &"C".repeat(num / 100); num %= 100; }
    if num >= 90   { result += "XC"; num -= 90; }
    if num >= 50   { result += "L"; num -= 50; }
    if num >= 40   { result += "XL"; num -= 40; }
    if num >= 10   { result += &"X".repeat(num / 10); num %= 10; }
    if num >= 9    { result += "IX"; num -= 9; }
    if num >= 5    { result += "V"; num -= 5; }
    if num >= 4    { result += "IV"; num -= 4; }
    if num >= 1    { result += &"I".repeat(num); }

    result
}

/// Find the number of characters saved when the given numerals are all rewritten in minimal form.
fn solve(numerals: &[String]) -> usize {
    let minimal_numerals = numerals.iter().map(|numeral| write_numeral(read_numeral(numeral)));
    let savings = numerals.iter().zip(minimal_numerals).map(|(n1, n2)| n1.len() - n2.len());
    savings.sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem089.txt")).unwrap();
    let reader = BufReader::new(file);
    let numerals: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    solve(&numerals).to_string()
}

problem!(answer, "743");
