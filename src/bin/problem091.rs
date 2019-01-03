//! [Problem 91 (Right triangles with integer coordinates)](https://projecteuler.net/problem=91)
//!
//! # Solution detail
//!
//! The problem space is small enough to brute-force, but we can do much better.
//!
//! To start with, for axis-aligned triangles, there are 3N^2 possible right triangles; N^2 for
//! each choice of which vertex should have the right-angle. We can ignore these and then add on
//! 3N^2 at the end.
//!
//! For the remaining triangles, with no points (other than the origin) on either axis, we can
//! count them by first fixing the vertex with the right angle. Suppose that P (x, y) is the vertex
//! with the right angle. Then we require the gradient from P to Q to be equal to (-x / y).
//!
//! So to count the possibilities for Q, given a fixed P, we can reduce this fraction (-x / y) and
//! count how many lattice points lie on that gradient from P within the required bounds. Suppose
//! the gradient reduces to (-a / b). Then we get points Q by moving `a` steps down and `b` steps
//! right `k` times, where `k` may be either positive or negative.
//!
//! The minimum allowed value for `k` is `-min(x/b, (N-y)/a)` and the maximum is `min((N-x)/n, y/a).
//! The difference between these two is the number of choices for Q (we can't have k = 0).
//!
//! In summary, we can iterate over all choices for P, count the possibilities for Q, and sum over
//! all P, remembering to add on 3N^2 at the end.

use itertools::iproduct;
use number_theory::gcd;
use projecteuler_rs::problem;
use std::cmp::min;

/// Count the number of third vertices that, along with the origin and the given vertex, form a
/// right-angled triangle within the boudns 0 ≤ x, y ≤ n
fn count_third_vertices(n: u64, x: u64, y: u64) -> u64 {
    let g = gcd(x, y);
    let (a, b) = (x / g, y / g);
    min(x / b, (n - y) / a) + min((n - x) / b, y / a)
}

/// Find the number of right-angled triangles having their coordinates within the box 0 ≤ x, y ≤ N
fn solve(grid_size: u64) -> u64 {
    let not_axis_aligned: u64 = iproduct!(1..grid_size + 1, 1..grid_size + 1)
        .map(|(x, y)| count_third_vertices(grid_size, x, y))
        .sum();
    not_axis_aligned + 3 * grid_size * grid_size
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(50).to_string()
}

problem!(answer, "14234");
