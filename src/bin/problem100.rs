//! [Problem 100 (Arranged probability)](https://projecteuler.net/problem=100)
//!
//! # Solution detail
//!
//! Consider a box with `b` blue discs and `r` red discs. The probability of drawing two blue discs
//! at random is equal to:
//!
//! `(b / (b + r)) * ((b - 1) / (b + r - 1))
//!
//! Setting this equal to 1/2, we get an equation that expands out to:
//!
//! `2b(b - 1) = (b + r)(b + r - 1)`
//!
//! Setting `b + r = n` and expanding even further, we get the equation, after rearrangement:
//!
//! `(2n - 1)² - 2(2b - 1)² = -1`
//!
//! Therefore, all solutions for `(b, r)` can be derived from the solutions `(x, y)` to the Pell
//! equation `x² - 2y² = -1`, where `x` and `y` must be odd. Thus we can simply iterate over all
//! solutions to that equation, which are given by starting with the base solution `(x, y) = (1, 1)`
//! and iterating using the map:
//!
//! `(x, y) --> (3x + 4y, 2x + 3y)`
//!
//! Apply this map repeatedly until we find a solution for which `b + r` is large enough. The
//! information we need is derived using `b + r = (x + 1) / 2` and `b = (y + 1) / 2`.

use itertools::iterate;
use projecteuler_rs::problem;

/// Find the sum of perimeters of all almost-equilateral triangles with integer area, where the
/// perimeters of said triangles do not exceed the given limit.
fn solve(limit: u64) -> u64 {
    let pell_solutions = iterate((1, 1), |&(x, y)| (3 * x + 4 * y, 2 * x + 3 * y));
    for (x, y) in pell_solutions {
        if (x + 1) / 2 > limit {
            return (y + 1) / 2;
        }
    }
    unreachable!();
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000_000_000).to_string()
}

problem!(answer, "756872327473");
