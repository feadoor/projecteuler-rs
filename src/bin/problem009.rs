//! [Problem 9 (Special Pythagorean triplet)](https://projecteuler.net/problem=9)
//!
//! # Solution detail
//!
//! There's no need to try to be clever here - brute force works just fine. Loop over `a`, `b` and
//! check whether `(a, b, 1000 - a - b)` is a Pythagorean triple.
//!
//! The answer can also be obtained with pen-and-paper by noticing that `a`, `b` satisfy the
//! condition exactly when `(1000 - a)(1000 - b) = 500000`.The only factorisation which leads to a
//! valid solution is then `500000 = 800 * 625` which directly gives `(a, b, c) = (200, 375, 425)`.

use projecteuler_rs::problem;

/// Find the first Pythagorean triplet with a + b + c = n, returning the product abc.
fn solve(n: u64) -> u64 {
    for a in 1..n {
        for b in a + 1..n - a {
            let c = n - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }

    0
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "31875000");
