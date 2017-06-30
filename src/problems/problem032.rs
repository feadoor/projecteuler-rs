//! [Problem 32 (Pandigital products)](https://projecteuler.net/problem=32)
//!
//! # Problem statement
//!
//! We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
//! exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
//! multiplier, and product is 1 through 9 pandigital.
//!
//! Find the sum of all products whose multiplicand/multiplier/product identity can be written as
//! a 1 through 9 pandigital.
//!
//! HINT: Some products can be obtained in more than one way so be sure to only include it once in
//! your sum.
//!
//! # Solution detail
//!
//! Note that for `a * b = c` to be a 1-9 pandigital product, where `a < b`, there are only two
//! possibilities for the lengths of the operands:
//!
//! <ul>
//! <li>(2 digits) * (3 digits) = (4 digits)</li>
//! <li>(1 digit)  * (4 digits) = (4 digits)</li>
//! </ul>
//!
//! These two cases are small enough that we can just iterate over all choices for `a, b` and
//! check if the resulting product is pandigital.

use std::collections::HashSet;

/// The name of the problem.
pub const NAME: &'static str = "Problem 32";
/// A description of the problem.
pub const DESC: &'static str = "Pandigital products";

/// Check whether the given numbers are together pandigital.
fn is_pandigital(nums: &mut [u64]) -> bool {

    // Keep track of the digits seen so far between the numbers.
    let mut seen = vec![false; 10];

    // For each given number, check its digits and mark them as seen. Return false if we have
    // already seen one of the digits.
    for num in nums.iter_mut() {
        while *num != 0 {
            let digit = *num % 10;
            *num /= 10;
            if seen[digit as usize] {
                return false;
            } else {
                seen[digit as usize] = true;
            }
        }
    }

    // We need to have seen every digit except zero.
    !seen[0] && seen[1..].iter().all(|&x| x)
}

/// Find the sum of all pandigital products.
fn solve() -> u64 {
    // Keep track of the products that have been found so far.
    let mut products = HashSet::new();

    // Check `a` having 2 digits and `b` having 3 digits.
    for a in 12..100 {
        for b in 123..1000 {
            let c = a * b;
            if is_pandigital(&mut [a, b, c]) {
                products.insert(c);
            }
        }
    }

    // Check `a` having 1 digit and `b` having 4 digits.
    for a in 1..10 {
        for b in 1234..10000 {
            let c = a * b;
            if is_pandigital(&mut [a, b, c]) {
                products.insert(c);
            }
        }
    }

    products.iter().sum()
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem032() {
        assert_eq!(super::answer(), "45228");
    }
}
