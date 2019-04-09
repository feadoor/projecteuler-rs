//! [Problem 121 (Disc game prize fund)](https://projecteuler.net/problem=121)
//!
//! # Solution detail
//!
//! Let `f(k, b)` be the number of ways (treating all discs as distinct) that there are of drawing
//! exactly `b` vlue discs after `k` turns of the game.
//!
//! The base case where `k = 0` is trivial: `f(0, 0) = 1` and all other `f(0, b) = 0`.
//!
//! We can calculate the values for larger `k` inductively using the formula:
//!
//! `f(k + 1, b) = k * f(k, b) + f(k, b - 1)`
//!
//! This formula follows from the fact that on turn `k + 1` there are `k` red discs and only 1 blue
//! disc.
//!
//! We can therefore use this formula to calculate all values `f(n, b)` for a particular `n`, and
//! from there, the probability is easy to calculate. We can ease memory requirements by using a
//! single array to store the values for each `k` in sequence.
//!
//! If the probability of drawing more blue discs
//! than red discs is x / y, then the maximum prize fund should be floor(y, x).

use projecteuler_rs::problem;

/// Find the maximum prize fund that should be allocated for a game lasting `n` turns.
fn solve(n: usize) -> usize {

    // Set up the base case
    let mut arr = vec![0; n + 1]; arr[0] = 1;

    // Iterate over each number of turns
    for k in 1..n + 1 {
        for idx in (1..k + 1).rev() {
            arr[idx] = k * arr[idx] + arr[idx - 1];
        }
        arr[0] = k * arr[0];
    }

    // Calculate the final probability
    let numer: usize = arr[n/2 + 1..].iter().sum();
    let denom: usize = arr.iter().sum();
    denom / numer
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(15).to_string()
}

problem!(answer, "2269");
