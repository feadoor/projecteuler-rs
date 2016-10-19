//! [Problem 9 (Special Pythagorean triplet)](https://projecteuler.net/problem=9)
//!
//! # Problem statement
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which
//! a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup>
//!
//! For example, 3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup>.
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
//!
//! # Solution detail
//!
//! There's no need to try to be clever here - brute force works just fine. Loop over `a`, `b` and
//! check whether `(a, b, 1000 - a - b)` is a Pythagorean triple.
//!
//! The answer can also be obtained with pen-and-paper by noticing that `a`, `b` satisfy the
//! condition exactly when `(1000 - a)(1000 - b) = 500000`.The only factorisation which leads to a
//! valid solution is then `500000 = 800 * 625` which directly gives `(a, b, c) = (200, 375, 425)`.

/// The name of the problem.
pub const NAME: &'static str = "Problem 9";
/// A description of the problem.
pub const DESC: &'static str = "Special Pythagorean triplet";

/// Find the first Pythagorean triplet with a + b + c = n, returning the product abc.
fn solve(n: u64) -> u64 {
    for a in 1..n {
        for b in a + 1..n - a {
            let c = n - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }

    0
}

/// Solve the problem, returning the answer as a `String`
pub fn answer() -> String {
    solve(1000).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem009() {
        assert_eq!(super::answer(), "31875000");
    }
}
