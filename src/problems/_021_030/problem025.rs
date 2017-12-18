//! [Problem 25 (1000-digit Fibonacci number)](https://projecteuler.net/problem=25)
//!
//! # Problem statement
//!
//! The Fibonacci sequence is defined by the recurrence relation:
//!
//! F<sub>n</sub> = F<sub>n−1</sub> + F<sub>n−2</sub>, where F<sub>1</sub> = 1 and
//! F<sub>2</sub> = 1.
//!
//! Hence the first 12 terms will be:
//!
//! F<sub>1</sub> = 1<br>
//! F<sub>2</sub> = 1<br>
//! F<sub>3</sub> = 2<br>
//! F<sub>4</sub> = 3<br>
//! F<sub>5</sub> = 5<br>
//! F<sub>6</sub> = 8<br>
//! F<sub>7</sub> = 13<br>
//! F<sub>8</sub> = 21<br>
//! F<sub>9</sub> = 34<br>
//! F<sub>10</sub> = 55<br>
//! F<sub>11</sub> = 89<br>
//! F<sub>12</sub> = 144
//!
//! The 12th term, F<sub>12</sub>, is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
//!
//! # Solution detail
//!
//! We don't actually need to calculate any Fibonacci numbers for this, which is a relief, since
//! it's nice to avoid dealing with 1000-digit numbers if we can help it. Since F<sub>n</sub> is
//! the closest integer to `(phi ^ n) / √5`, we just need to find the first n for which:
//!
//! `n * log_10(ϕ) > 1000 + log_10(√5) - 1`

#[macro_use]
extern crate projecteuler_rs;

/// Find the index of the first Fibonacci number with at least n digits.
fn solve(n: u64) -> u64 {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let rhs = n as f64 - 1.0 + 5f64.sqrt().log(10.0);
    (rhs / phi.log(10.0)).ceil() as u64
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "4782");
