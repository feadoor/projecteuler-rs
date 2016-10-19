//! [Problem 1 (Multiples of 3 and 5)](https://projecteuler.net/problem=1)
//!
//! # Problem statement
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! # Solution detail
//!
//! Using the
//! [inclusion-exclusion principle](https://en.wikipedia.org/wiki/Inclusion–exclusion_principle),
//! we can calculate the required sum as `s_3 + s_5 - s_15`, where:
//!
//! `s_3` is the sum of the multiples of 3 below 1000.
//!
//! `s_5` is the sum of the multiples of 5 below 1000.
//!
//! `s_15` is the sum of the multiples of 15 (i.e. divisible by both 3 and 5) below 1000.
//!
//! Each of these quantites is the sum of an
//! [arithmetic progression](https://en.wikipedia.org/wiki/Arithmetic_progression) and can be
//! calculated by the usual formula.

/// The name of the problem.
pub const NAME: &'static str = "Problem 1";
/// A description of the problem.
pub const DESC: &'static str = "Multiples of 3 and 5";

/// Calculate the sum of the multiples of `n` below the given limit, using the formula for the sum
/// of an arithmetic progression.
fn sum_of_multiples(n: u64, limit: u64) -> u64 {
    let num_terms = (limit - 1) / n;
    n * num_terms * (num_terms + 1) / 2
}

/// Find the sum of all multiples of 3 or 5 below the given limit.
fn solve(limit: u64) -> u64 {
    let s_3 = sum_of_multiples(3, limit);
    let s_5 = sum_of_multiples(5, limit);
    let s_15 = sum_of_multiples(15, limit);

    s_3 + s_5 - s_15
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem001() {
        assert_eq!(super::answer(), "233168");
    }
}
