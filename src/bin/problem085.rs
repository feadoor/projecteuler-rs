//! [Problem 85 (Counting rectangles)](https://projecteuler.net/problem=85)
//!
//! # Solution detail
//!
//! For an m × n grid, the number of sub-rectangles is given by T<sub>m</sub> × T<sub>n</sub>, where
//! T<sub>k</sub> = k * (k + 1) / 2 is the kth triangular number.

//! This is because there are T<sub>m</sub> choices for the projection of the rectangle onto the
//! x-axis, similarly T<sub>n</sub> choices for the projection onto the y-axis, and these
//! projections uniquely determing the rectangle.
//!
//! This means that we just need to generate the triangular numbers, up to and including the first
//! one which exceeds the target number of sub-rectangles, and for each one, find the triangular
//! number that pairs with it to give the product closest to the target.
//!
//! Doing this for each triangular number, and keeping track of the best rectangle so far, we will
//! arrive at the answer.

#[macro_use]
extern crate projecteuler_rs;
extern crate itertools;

use itertools::Itertools;

/// Calculate the nth triangular number
fn triangle(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Find the triangular numbers up to and including the first one which is at least `target`.
fn triangles_until(target: u64) -> Vec<u64> {
    (0..).map(triangle)
        .tuple_windows::<(_, _)>()
        .take_while(|x| x.0 < target)
        .map(|x| x.1)
        .collect()
}

/// Find the index of the first entry in the (sorted!) vector which is greater than the target.
fn first_index_greater_than(entries: &[u64], target: u64) -> usize {
    let (mut lo, mut hi) = (0, entries.len() - 1);
    while lo != hi {
        let mid = (lo + hi) / 2;
        if entries[mid] > target {
            hi = mid
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Find the area of the rectangle with a number of sub-rectangles closest to the given target.
fn solve(target: u64) -> usize {
    let triangular_numbers = triangles_until(target);
    let (mut best_area, mut best_distance) = (0, u64::max_value());

    macro_rules! check {
        ($m:expr, $n:expr) => {
            let result = triangular_numbers[$m] * triangular_numbers[$n];
            let distance = if result > target { result - target } else { target - result };
            if distance < best_distance {
                best_area = ($m + 1) * ($n + 1);
                best_distance = distance;
            }
        }
    }

    for m in 0..triangular_numbers.len() {
        let n = first_index_greater_than(&triangular_numbers, target / triangular_numbers[m]);
        if n < triangular_numbers.len() { check!(m, n); }
        if n > 0 { check!(m, n - 1); }
    }

    best_area
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(2_000_000).to_string()
}

problem!(answer, "2772");
