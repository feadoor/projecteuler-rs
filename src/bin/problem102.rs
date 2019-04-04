//! [Problem 102 (Triangle containment)](https://projecteuler.net/problem=102)
//!
//! # Solution detail
//!
//! To check if a triangle ABC contains a point P, we need to know two facts:
//!
//!   1. P is within the triangle iff the signed areas of triangles ABP, BCP and CAP all have the
//!   same non-zero sign.
//!
//!   2. The signed area of triangle XYZ with coordinates `(x₁, y₁), (x₂, y₂), (x₃, y₃)` is equal to
//!   the determinant of the 3x3 matrix having `(xᵢ, yᵢ, 1)` as rows.
//!
//! Thus, to check that a triangle ABC contains the origin, we simply need to calculate the signum
//! of the signed areas of triangles `ABO`, `BCO`, `CAO` and check that they are all equal.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use projecteuler_rs::problem;

type Point = (isize, isize);
type Triangle = (Point, Point, Point);

/// Check if the given triangle contains the origin.
fn contains_origin(triangle: &Triangle) -> bool {
    let &((x1, y1), (x2, y2), (x3, y3)) = triangle;

    let sign1 = (x1 * y2 - x2 * y1).signum();
    let sign2 = (x2 * y3 - x3 * y2).signum();
    let sign3 = (x3 * y1 - x1 * y3).signum();

    sign1 == sign2 && sign2 == sign3 && sign1 != 0
}

/// Find the count of the given triangles which contain (strictly) the origin.
fn solve(triangles: &[Triangle]) -> usize {
    triangles.iter().filter(|t| contains_origin(t)).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    let file = File::open(&Path::new("inputs/problem102.txt")).unwrap();
    let reader = BufReader::new(file);
    let triangles: Vec<_> = reader.lines()
        .map(|line| line.unwrap().split(',').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .map(|coords| ((coords[0], coords[1]), (coords[2], coords[3]), (coords[4], coords[5])))
        .collect();
    solve(&triangles).to_string()
}

problem!(answer, "228");
