//! [Problem 18 (Maximum path sum I)](https://projecteuler.net/problem=18)
//!
//! # Problem statement
//!
//! By starting at the top of the triangle below and moving to adjacent numbers on the row below,
//! the maximum total from top to bottom is 23.
//!
//! ```text
//!    𝟑
//!   𝟕 4
//!  2 𝟒 6
//! 8 5 𝟗 3
//! ```
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom of the triangle below:
//!
//! ```text
//!                             75
//!                           95  64
//!                         17  47  82
//!                       18  35  87  10
//!                     20  04  82  47  65
//!                   19  01  23  75  03  34
//!                 88  02  77  73  07  63  67
//!               99  65  04  28  06  16  70  92
//!             41  41  26  56  83  40  80  70  33
//!           41  48  72  33  47  32  37  16  94  29
//!         53  71  44  65  25  43  91  52  97  51  14
//!       70  11  33  28  77  73  17  78  39  68  17  57
//!     91  71  52  38  17  14  91  43  58  50  27  29  48
//!   63  66  04  68  89  53  67  30  73  16  69  87  40  31
//! 04  62  98  27  23  09  70  98  73  93  38  53  60  04  23
//! ```
//!
//! NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every
//! route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows;
//! it cannot be solved by brute force, and requires a clever method! ;o)
//!
//! # Solution detail
//!
//! This is a classic example of a problem which can be solved with dynamic programming. For each
//! cell in the triangle, we will calculate the maximum value of a path ending at that cell. The
//! solution is then the largest of these numbers.
//!
//! To calculate these numbers, start at the top of the triangle and work down. For the apex, the
//! maximum path sum is simply the value in that cell. For every other cell, the maximum path sum
//! is the number in that cell, plus the greater of the two maximums from the cells directly above
//! it.

use std::cmp::max;

/// The name of the problem.
pub const NAME: &'static str = "Problem 18";
/// A description of the problem.
pub const DESC: &'static str = "Maximum path sum I";

/// Find the maximum sum of a path through the given triangular grid, given as a vector of vectors
/// and guaranteed to be of valid dimensions.
fn solve(grid: &[Vec<u64>]) -> u64 {

    // Keep track of the maximum path sums from the previous row.
    let mut prev_row = vec![0; grid.len()];
    prev_row[0] = grid[0][0];

    // Iterate over the rows of the triangle and find the maximum sums for the next row.
    for ix in 1..grid.len() {
        let mut curr_row = vec![0; grid.len()];
        for jx in 0..ix + 1 {
            let left = if jx > 0 { prev_row[jx - 1] } else { 0 };
            let right = if jx < ix { prev_row[jx] } else { 0 };
            curr_row[jx] = grid[ix][jx] + max(left, right);
        }
        prev_row = curr_row;
    }

    *prev_row.iter().max().unwrap()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    let grid_str = "\
        75
        95 64
        17 47 82
        18 35 87 10
        20 04 82 47 65
        19 01 23 75 03 34
        88 02 77 73 07 63 67
        99 65 04 28 06 16 70 92
        41 41 26 56 83 40 80 70 33
        41 48 72 33 47 32 37 16 94 29
        53 71 44 65 25 43 91 52 97 51 14
        70 11 33 28 77 73 17 78 39 68 17 57
        91 71 52 38 17 14 91 43 58 50 27 29 48
        63 66 04 68 89 53 67 30 73 16 69 87 40 31
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    let parse_row = |x: &str| -> Vec<u64> {
        x.split_whitespace().map(|x| u64::from_str_radix(x, 10).unwrap()).collect()
    };
    let grid: Vec<Vec<u64>> = grid_str.split('\n')
        .map(parse_row)
        .collect();

    solve(&grid).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem018() {
        assert_eq!(super::answer(), "1074");
    }
}
