//! [Problem 75 (Digit factorial chains)](https://projecteuler.net/problem=74)
//!
//! # Problem statement
//!
//! It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided
//! right angle triangle in exactly one way, but there are many more examples.
//!
//! 12 cm: (3, 4, 5)
//! 24 cm: (6, 8, 10)
//! 30 cm: (5, 12, 13)
//! 36 cm: (9, 12, 15)
//! 40 cm: (8, 15, 17)
//! 48 cm: (12, 16, 20)
//!
//! In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right
//! angle triangle, and other lengths allow more than one solution to be found; for example, using
//! 120 cm it is possible to form exactly three different integer sided right angle triangles.
//!
//! 120 cm: (30, 40, 50), (20, 48, 52), (24, 45, 51)
//!
//! Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one
//! integer sided right angle triangle be formed?
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

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use number_theory::primitive_pythag_triples;

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
