//! [Problem 53 (Combinatoric selections)](https://projecteuler.net/problem=53)
//!
//! # Solution detail
//!
//! We can use two facts about binomial coefficients to speed up the search:
//!
//!  - <sup>n</sup>C<sub>r</sub> is increasing for 0 ≤ <i>r</i> ≤ <i>n</i> / 2
//!  - <sup>n</sup>C<sub>n-r</sub> = <sup>n</sup>C<sub>r</sub>
//!
//! This means that, for each <i>n</i>, we can iterate up through the values
//! 0 ≤ <i>r</i> ≤ <i>n</i> / 2, and when we find <sup>n</sup>C<sub>r</sub> > 1,000,000 for the
//! first time, this means that there are <i>n</i> - 2<i>r</i> + 1 values of <i>r</i> which cause
//! the binomial coefficient to exceed one million.
//!
//! The values of <sup>n</sup>C<sub>r</sub> for increasing <i>r</i> can be calculated using the
//! relation <sup>n</sup>C<sub>r+1</sub> =
//! <sup>n</sup>C<sub>r</sub> × (<i>n</i> - <i>r</i>) / (<i>r</i> + 1)

use projecteuler_rs::problem;

/// A structure which will allow iteration over binomial coefficients.
struct BinomialCoefficientsWithNumerator {
    /// The value of the current coefficient.
    value: u64,
    /// The numerator of the current coefficient.
    numer: u64,
    /// The denominator of the current coefficient.
    denom: u64,
    /// The highest denominator to consider for these coefficients.
    denom_limit: u64,
}

impl Iterator for BinomialCoefficientsWithNumerator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.denom < self.denom_limit {
            let old_value = self.value;
            self.value = (self.value * (self.numer - self.denom)) / (self.denom + 1);
            self.denom += 1;

            Some(old_value)
        } else {
            None
        }
    }
}

/// Returns an iterator of the binomials coefficients n choose r, with 0 ≤ r ≤ n / 2.
fn binomial_coefficients_lower_half(n: u64) -> BinomialCoefficientsWithNumerator {
    BinomialCoefficientsWithNumerator {
        value: 1,
        numer: n,
        denom: 0,
        denom_limit: n / 2,
    }
}

/// Returns the smallest r for which n choose r exceeds the given threshold, if it exists.
fn first_binomial_coefficient_to_exceed(n: u64, threshold: u64) -> Option<u64> {
    binomial_coefficients_lower_half(n).position(|x| x > threshold).map(|val| val as u64)
}

/// Find the number of binomial coefficients n choose r, with n not exceeding the given limit, whose
/// value is greater than the given threshold.
fn solve(limit: u64, threshold: u64) -> u64 {
    (1..limit + 1)
        .filter_map(|n| first_binomial_coefficient_to_exceed(n, threshold).map(|r| n - 2 * r + 1))
        .sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(100, 1_000_000).to_string()
}

problem!(answer, "4075");
