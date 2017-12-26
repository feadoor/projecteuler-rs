//! [Problem 68 (Magic 5-gon ring)](https://projecteuler.net/problem=68)
//!
//! # Problem statement
//!
//! Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding
//! to nine.
//!
//! ```text
//!   4
//!    \
//!     3
//!    / \
//!   1---2---6
//!  /
//! 5
//! ```
//!
//! Working clockwise, and starting from the group of three with the numerically lowest external
//! node (4,3,2 in this example), each solution can be described uniquely. For example, the above
//! solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
//!
//! It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are
//! eight solutions in total.
//!
//! | Total | Solution Set        |
//! |-------|---------------------|
//! | 9     | 4,2,3; 5,3,1; 6,1,2 |
//! | 9     | 4,3,2; 6,2,1; 5,1,3 |
//! | 10    | 2,3,5; 4,5,1; 6,1,3 |
//! | 10    | 2,5,3; 6,3,1; 4,1,5 |
//! | 11    | 1,4,6; 3,6,2; 5,2,4 |
//! | 11    | 1,6,4; 5,4,2; 3,2,6 |
//! | 12    | 1,5,6; 2,6,4; 3,4,5 |
//! | 12    | 1,6,5; 3,5,4; 2,4,6 |
//!
//! By concatenating each group it is possible to form 9-digit strings; the maximum string for a
//! 3-gon ring is 432621513.
//!
//! Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and
//! 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?
//!
//! # Solution detail
//!
//! This problem is an example of one which can be solved much, much more easily by hand than by
//! writing a program.
//!
//! In order to maximise the first digit of the solution string, we should put the largest numbers
//! around the edge of the ring - 6, 7, 8 9 and 10. The sum of all 5 rows will then be:
//!
//! `6 + 7 + 8 + 9 + 10 + 2 * (1 + 2 + 3 + 4 + 5) = 70`
//!
//! This means that each row must have a total of 14. In order to maximise the solution string, the
//! digit appearing after 6 should be as large as possible, that is, 5. Then the next digit must be
//! 3, to achieve the total of 14. This leaves us with:
//!
//! ```text
//!       6
//!        \
//!  E--J---5   B
//!    /     \ /
//!   I       3
//!  / \     /
//! D   \   /
//!      \ /
//!       H
//!        \
//!         C
//! ```
//!
//! Wherever the 10 goes in the outer ring, it must be paired with 3 and 1 in the inner ring to
//! achieve the total of 14. This forces the next two numbers in the ring:
//!
//! ```text
//!       6
//!        \
//!  E--J---5   10
//!    /     \ /
//!   I       3
//!  / \     /
//! D   \   /
//!      \ /
//!       1
//!        \
//!         C
//! ```
//!
//! The digit 2 cannot appear next to 1, since there is no 11, which would be needed to make 14.
//! This allows us to complete the inner ring:
//!
//! ```text
//!       6
//!        \
//!  E--2---5   10
//!    /     \ /
//!   4       3
//!  / \     /
//! D   \   /
//!      \ /
//!       1
//!        \
//!         C
//! ```
//!
//! Finally, the numbers in the outer ring must be placed so as to maintain totals of 14 for each of
//! the rows:
//!
//! ```text
//!       6
//!        \
//!  7--2---5   10
//!    /     \ /
//!   4       3
//!  / \     /
//! 8   \   /
//!      \ /
//!       1
//!        \
//!         9
//! ```
//!
//! This is indeed a solution, and by construction, must be the maximal solution.

#[macro_use]
extern crate projecteuler_rs;

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    "6531031914842725".to_string()
}

problem!(answer, "6531031914842725");
