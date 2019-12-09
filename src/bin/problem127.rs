//! [Problem 127 (abc-hits)](https://projecteuler.net/problem=127)
//!
//! # Solution detail
//!
//! Firstly, we'll be computing a lot of radicals. It will save us a lot of time to have a
//! precomputed list of them ready to hand - we only need them for inputs up to the limit of the
//! problem. This list can be computed using a modified Sieve of Eratosthenes.
//!
//! It will also be convenient to have a list of inputs sorted by radical - the reason will
//! become clear in a moment.
//!
//! For a given abc-hit, it is true that all of a, b and c are pairwise coprime, so that rad(abc)
//! is equal to rad(a) * rad(b) * rad(c) which saves us some computation. It is also true that
//! rad(a) = rad(abc) / rad(bc) < c / rad(bc) < b / rad(b), where the final inequality is true
//! because c < 2b and rad(c) >= 2.
//!
//! This leads to the following algorithm. Iterate over all possible b. For each one, iterate over
//! every a with gcd(a, b) = 1 and rad(a) < b / rad(b), and check if we have an abc-hit. The
//! iteration over a is why it is useful to have values sorted by their radical.

use number_theory::gcd;
use projecteuler_rs::problem;

/// Compute a list of radicals of all numbers less than the given limit.
fn all_radicals(limit: usize) -> Vec<usize> {
    let mut result = vec![1; limit];
    for p in 2..limit {
        if result[p] == 1 {
            for multiple in (p..limit).step_by(p) {
                result[multiple] *= p;
            }
        }
    }
    result
}

/// Determine all abc-hits with c less than the given limit.
fn abc_hits(limit: usize) -> Vec<(usize, usize, usize)> {

    let mut hits = Vec::new();

    // Precomputed radicals for easy lookup, and inputs sorted by their radical.
    let radicals = all_radicals(limit);
    let mut inputs: Vec<_> = (1..limit).collect();
    inputs.sort_by_key(|&ix| radicals[ix]);

    // Iterate over possible choices for b, then a based on b
    for b in 1..limit {
        for &a in inputs.iter().take_while(|&&a| radicals[a] < b / radicals[b]).filter(|&&a| a < b && a + b < limit && gcd(a, b) == 1) {
            if radicals[a] * radicals[b] * radicals[a + b] < a + b {
                hits.push((a, b, a + b));
            }
        }
    }

    hits
}

/// Find the sum of `c` in al abc-hits with `c` less than the limit.
fn solve(limit: usize) -> usize {
    abc_hits(limit).iter().map(|(_a, _b, c)| c).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(120000).to_string()
}

problem!(answer, "18407904");
