//! [Problem 39 (Integer right triangles)](https://projecteuler.net/problem=39)
//!
//! # Problem statement
//!
//! If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
//! exactly three solutions for p = 120:
//!
//! {20,48,52}, {24,45,51}, {30,40,50}
//!
//! For which value of p ≤ 1000, is the number of solutions maximised?
//!
//! # Solution detail
//!
//! Simply iterate over all Pythagorean triples with perimeter at most 1000. Keep track, for each
//! possible perimeter, how many triangles with that perimeter have been found, and then just
//! find the perimeter which has the most.
//!
//! To iterate over Pythagorean triples, find the primitive triples using the tree structure
//! described [here](https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples), and
//! multiply up to non-primitive triples as well.

#[macro_use]
extern crate projecteuler_rs;
extern crate number_theory;

use number_theory::primitive_pythag_triples;

/// Find the perimeter below the given limit for which there are the most right-angled triangles.
fn solve(limit: u64) -> u64 {
    // Keep track of how many triangles have been found with each perimeter.
    let mut counts = vec![0; limit as usize + 1];

    // Iterate over the primitive Pythagorean triples.
    for (a, b, c) in primitive_pythag_triples(|(a, b, c)| a + b + c <= limit) {
        let perimeter = a + b + c;
        for k in 1..limit / perimeter + 1 {
            counts[(k * perimeter) as usize] += 1;
        }
    }

    (1..limit + 1).max_by_key(|&x| counts[x as usize]).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(1_000).to_string()
}

problem!(answer, "840");
