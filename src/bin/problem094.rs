//! [Problem 94 (Almost equilateral triangles)](https://projecteuler.net/problem=94)
//!
//! # Solution detail
//!
//! Let the common side of the triangle be `a`, so that `b = a ± 1` is the other side. Then the
//! length `h` of the altitude from the isosceles vertex is given by:
//!
//! `4h² = 4a² - b²`
//!
//! Setting `b = a ± 1` and rearranging, we get the equation:
//!
//! `((3a ± 1) / 2)² - 3h² = 1`
//!
//! Thus, all solutions for `a`, `b` can be derived from the solutions to the Pell equation
//! `x² - 3y² = 1`. Given a solution to this equation, `a` and `b` are given by:
//!
//! If `x ≡ 1 (mod 3)` then `a = (2x + 1) / 3` and `b = a + 1`
//!
//! If `x ≡ 2 (mod 3)` then `a = (2x - 1) / 3` and `b = a - 1`
//!
//! This Pell equation has fundamental solution `(x, y) = (2, 1)` and additional solutions can be
//! generated using the transformation `(x, y) --> (2x + 3y, x + 2y)`.
//!
//! So to solve the problem, simply generate all solutions to this Pell equation iteratively, find
//! the corresponding perimeters, and stop when the solutions get too large. Remember not to count
//! the solution that comes from `(x, y) = (2, 1)` - this is `(1, 1, 0)` which is not a real
//! triangle.

use itertools::iterate;
use projecteuler_rs::problem;

/// Find the sum of perimeters of all almost-equilateral triangles with integer area, where the
/// perimeters of said triangles do not exceed the given limit.
fn solve(limit: u64) -> u64 {
    let pell_solutions = iterate((2, 1), |&(x, y)| (2 * x + 3 * y, x + 2 * y)).skip(1);
    let perimeters = pell_solutions.map(|(x, _)| match x % 3 {
        1 => 2 * (x + 1),
        2 => 2 * (x - 1),
        _ => unreachable!(),
    });
    perimeters.take_while(|&p| p <= limit).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000_000_000).to_string()
}

problem!(answer, "518408346");
