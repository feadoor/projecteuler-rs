//! [Problem 75 (Singular integer right triangles)](https://projecteuler.net/problem=75)
//!
//! # Solution detail
//!
//! Simply iterate over all Pythagorean triples with perimeter at most 1,500,000. Keep track, for
//! each possible perimeter, how many triangles with that perimeter have been found, and then just
//! count how many perimeters have exactly one corresponding triple.
//!
//! To iterate over Pythagorean triples, find the primitive triples using the tree structure
//! described [here](https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples), and
//! multiply up to non-primitive triples as well.

use number_theory::primitive_pythag_triples;
use projecteuler_rs::problem;

/// Find the number of perimeters below the given limit for which there is exactly one right-angled
/// triangle having the given perimeter.
fn solve(limit: u64) -> usize {
    // Keep track of how many triangles have been found with each perimeter.
    let mut counts = vec![0; limit as usize + 1];

    // Iterate over the primitive Pythagorean triples.
    for (a, b, c) in primitive_pythag_triples(|(a, b, c)| a + b + c <= limit) {
        let perimeter = a + b + c;
        for k in 1..limit / perimeter + 1 {
            counts[(k * perimeter) as usize] += 1;
        }
    }

    counts.iter().filter(|&&x| x == 1).count()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_500_000).to_string()
}

problem!(answer, "161667");
