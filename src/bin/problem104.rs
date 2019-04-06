//! [Problem 104 (Pandigital Fibonacci ends)](https://projecteuler.net/problem=104)
//!
//! # Solution detail
//!
//! Finding all Fibonacci numbers which are pandigital in the last 9 digits is fairly easy - simply
//! calculate the Fibonacci sequence using the usual iterative method, but only storing the terms
//! modulo 10^9.
//!
//! The real problem is, given an index `k` for which `Fₖ` is pandigital in the last 9 digits,
//! checking whether it is also pandigital in the first 9 digits. We surely cannot calculate the
//! full number, because the number of digits would be enormous.
//!
//! Instead, we can do some maths with logarithms to work out an approximation to `Fₖ` which is good
//! enough to be used to calculate the first 9 digits.
//!
//! Specifically, since for large `k` it is true that `Fₖ` is approximately equal to `ϕᵏ / √5`,
//! where `ϕ` is the golden ratio, we have the relationship:
//!
//! `First 9 digits of Fₖ ≈ 10^({ k * log₁₀(ϕ) - log₁₀(√5) }) * 10⁸`
//!
//! In the above, the braces `{}` represent the fractional part. This is fast and accurate enough
//! for our purposes.

use projecteuler_rs::problem;

/// Check whether the given number is pandigital.
fn is_pandigital(mut num: usize) -> bool {

    // Keep track of the digits seen in the number.
    let mut seen = vec![false; 10];

    while num != 0 {
        let digit = num % 10;
        num /= 10;
        if seen[digit] { return false; } else { seen[digit] = true; }
    }

    // We need to have seen every digit except zero.
    !seen[0] && seen[1..].iter().all(|&x| x)
}

/// Check whether the kth Fibonacci number is pandigital in the first 9 digits.
fn is_fibonacci_left_pandigital(k: usize) -> bool {
    let phi_log = ((1.0 + (5.0f64).sqrt()) / 2.0).log10();
    let fib_log = k as f64 * phi_log - (5.0f64).sqrt().log10();
    let fractional = fib_log.fract();

    is_pandigital(((10.0f64).powf(fractional) * 100_000_000 as f64) as usize)
}

/// Iterate over the Fibonacci numbers, modulo 10^9
struct FibonacciIterator {
    first: usize,
    second: usize,
}

impl FibonacciIterator {

    const MODULUS: usize = 1_000_000_000;

    fn new() -> Self {
        FibonacciIterator { first: 0, second: 1}
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_number = self.first;
        self.first = self.second; self.second = (self.first + next_number) % Self::MODULUS;
        Some(next_number)
    }
}

/// Find the index of the first Fibonacci number which is pandigital at both ends.
fn solve() -> usize {
    FibonacciIterator::new().enumerate()
        .filter(|&(k, fib)| is_pandigital(fib) && is_fibonacci_left_pandigital(k))
        .next().unwrap().0
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "329468");
