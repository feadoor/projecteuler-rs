//! [Problem 45 (Triangular, pentagonal and hexagonal)](https://projecteuler.net/problem=45)
//!
//! # Solution detail
//!
//! Numbers which are triangular and pentagonal are found by solving the equation
//! n × (n + 1) / 2 = m × (3m - 1) / 2. This can be put into the equivalent form:
//!
//! (6m - 1)<sup>2</sup> - 3 × (2n + 1)<sup>2</sup> = -2
//!
//! So we are searching for solutions (x, y) of the Pell-type equation
//! x<sup>2</sup> - 3y<sup>2</sup> = -2 which satisfy:
//!
//! x == 5 (mod 6)
//!
//! y == 1 (mod 2)
//!
//! Then m, n = (x + 1) / 6, (y - 1) / 2, and the figurate number is n × (n + 1) / 2
//!
//! The solutions to this Pell equation are easy to find - the fundamental solution is (1, 1)
//! and the generating solution is (2, 1), which means that we generate new solutions from old
//! using the formula (x, y) ⟶ (2x + 3y, x + 2y).
//!
//! For each solution we find, we can check whether the figurate number is also hexagonal, and if
//! so, we have a solution.

use number_theory::integer_sqrt;
use projecteuler_rs::problem;

/// A function to check if a given number is hexagonal.
///
/// Since hexagonal numbers are given by the formula Hₙ = 2n² - n, we can solve to get
/// n = (1 + sqrt(8Hₙ + 1)) / 4, so x is a hexagonal number iff 8x + 1 is a square and
/// (1 + sqrt(8x + 1)) is divisible by 4.
fn is_hexagonal(m: u64) -> bool {
    let n = 8 * m + 1;
    let sqrt = integer_sqrt(n);
    sqrt * sqrt == n && sqrt % 4 == 3
}

/// Find the kth number which is simultaneously triangular, pentagonal and hexagonal.
fn solve(k: usize) -> u64 {
    // Iterate up through successive solutions to the Pell equation x^2 - 3y^2 = -2
    let (mut x, mut y) = (1, 1);
    let (mut last_sol, mut sol_count) = (0, 0);
    while sol_count < k {

        // Check if x, y provide a solution to the original problem.
        if x % 6 == 5 && y % 2 == 1 {
            let n = (y - 1) / 2;
            let potential_sol = n * (n + 1) / 2;
            if is_hexagonal(potential_sol) {
                sol_count += 1;
                last_sol = potential_sol;
            }
        }

        // Move on to the next solution to the Pell equation.
        x = 2 * x + 3 * y;
        y = (x + y) / 2;
    }

    last_sol
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(3).to_string()
}

problem!(answer, "1533776805");
