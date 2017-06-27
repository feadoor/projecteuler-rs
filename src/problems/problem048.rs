//! [Problem 48 (Self powers)](https://projecteuler.net/problem=48)
//!
//! # Problem statement
//!
//! The series, 1<sup>1</sup> + 2<sup>2</sup> + 3<sup>3</sup> + ... + 10<sup>10</sup> = 10405071317.
//!
//! Find the last ten digits of the series, 1<sup>1</sup> + 2<sup>2</sup> + 3<sup>3</sup> + ... +
//! 1000<sup>1000</sup>.
//!
//! # Solution detail
//!
//! This is simply a case of using, say, a repeated squaring algorithm to calculate the necessary
//! powers modulo 10^10, and adding up the results.
//!
//! Since the modulus here is larger than 2^32, care must be taken that no intermediate
//! multiplication ever results in overflow.

use number_theory::modexp;

/// The name of the problem.
pub const NAME: &'static str = "Problem 48";
/// A description of the problem.
pub const DESC: &'static str = "Self powers";

/// Find the last 10 digits of the sum 1^1 + 2^2 + ... + n^n.
fn solve(n: u64) -> u64 {
    let modulus = 10_000_000_000;
    (1..n + 1).map(|x| modexp(x, x, modulus)).fold(0, |x, y| (x + y) % modulus)
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem048() {
        assert_eq!(super::answer(), "9110846700");
    }
}
