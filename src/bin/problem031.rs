//! [Problem 31 (Coin sums)](https://projecteuler.net/problem=31)
//!
//! # Solution detail
//!
//! Ignoring the story about the money, we are trying to solve the problem of figuring out how
//! many ways we can write a given number `n` (in this case, 200) as a sum of terms taken from a
//! particular set of values (in this case, 1,2,5,10,20,50,100,200).
//!
//! A very fast way of solving this problem, at the cost of `O(n)` memory, is to begin with an
//! array `a` initially equal to the following:
//!
//! ```
//! a[0] = 1
//! a[1] = 0
//! a[2] = 0
//! ...
//! a[n] = 0
//! ```
//!
//! Note that this initial assignment means that `a[i]` is the number of ways of writing `i` as a
//! sum of terms belonging to the empty set.
//!
//! We can then consider the values we are allowed to use in the sum (i.e. coin values) one at a
//! time, and update the values of the array `a` for each coin - at all times it should hold the
//! number of ways of making the values `0, 1, 2, ..., n` using the current set of coins.
//!
//! In pseudocode, this update is:
//!
//! ```
//! for j = coin_val to n:
//!     a[j] += a[j - coin_val]
//! ```
//!
//! After we have done this for all the allowed coin values, the answer is in `a[n]`.

use projecteuler_rs::problem;

/// Find the number of ways of making `n` pence using English coins.
fn solve(n: usize) -> u64 {

    // Create the array `a` which will hold the final answer in `a[n]`.
    let mut a = vec![0; n + 1];
    a[0] = 1;

    // For each coin value, update the array so that `a[i]` holds the number of ways of making
    // `i` using the coins considered so far.
    for &val in &[1, 2, 5, 10, 20, 50, 100, 200] {
        for j in val..n + 1 {
            a[j] += a[j - val];
        }
    }

    a[n]
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(200).to_string()
}

problem!(answer, "73682");
