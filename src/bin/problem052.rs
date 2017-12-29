//! [Problem 52 (Permuted multiples)](https://projecteuler.net/problem=52)
//!
//! # Solution detail
//!
//! For the condition to be satisfied, it must be the case that <i>x</i> is between 10...0 and
//! 16...6, for some number of digits, otherwise 6<i>x</i> would contain more digits than <i>x</i>.
//!
//! <i>x</i> must also contain at least 3 digits, otherwise there simply aren't enough permutations
//! of the digits to go round.
//!
//! We can therefore just iterate up through the possible values of <i>x</i>, and checking the
//! multiples of each one until we find a solution.

#[macro_use]
extern crate projecteuler_rs;

/// Convert the given number into a count of how many times each digit appears.
fn to_digit_count(mut n: u64) -> [usize; 10] {
    let mut counts = [0; 10];
    while n > 0 {
        counts[(n % 10) as usize] += 1;
        n /= 10;
    }
    counts
}

/// Find the smallest number x for which 2x, 3x..., 6x all contain the same digits as x.
fn solve() -> u64 {

    // Loop over the possible ranges 10...0 to 16...6 for varying numbers of digits.
    let mut lower = 100;
    let mut upper = 166;
    loop {

        // Search for a value satisfying the condition.
        'search: for x in lower..upper + 1 {
            let digit_counts = to_digit_count(x);
            for d in 2..7 {
                if to_digit_count(d * x) != digit_counts {
                    continue 'search;
                }
            }
            return x;
        }

        // Update the loop bounds and move on to the next range.
        lower *= 10;
        upper = 10 * upper + 6;
    }
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "142857");
