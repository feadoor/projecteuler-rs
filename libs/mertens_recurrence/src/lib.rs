//! This crate contains a framework for calculating isolated values of a function g(n) defined by
//! a recurrence relation of the form:
//!
//! sum[k = 1 to N] a(k) * g(N/k) = f(N)
//!
//! The crate is named `mertens_recurrence` because M(x), the so-called
//! [Mertens function](http://mathworld.wolfram.com/MertensFunction.html) is a very well-known
//! example of a function satisfying such a recurrence:
//!
//! sum[k = 1 to N] M(N / k) = 1
//!
//! If the function f(N) and the partial sums of coefficients a(k) can both be calculated quickly
//! (constant time), then isolated values of g(n) can be calculated in time O(n^(3/4)) and memory
//! O(n^(1/2)).

#![allow(non_snake_case)]
extern crate number_theory;

use number_theory::integer_sqrt;

/// A representation of a function defined using a Mertens-style recurrence relation.
pub struct Recurrence<F: Fn(u64) -> i64, A: Fn(u64, u64) -> i64> {
    /// The function f(N) from the right-hand side of the recurrence definition.
    f: F,
    /// The partial sum sum[k=b+1 to c] a(k) of the coefficients a(k) of the recurrence.
    a_partial_sum: A,
}

impl<F: Fn(u64) -> i64, A: Fn(u64, u64) -> i64> Recurrence<F, A> {

    /// Create a new `Recurrence` defined by the function `f` and the partial coefficient sums
    /// `a_partial_sum`.
    pub fn new(f: F, a_partial_sum: A) -> Recurrence<F, A> {
        Recurrence {
            f,
            a_partial_sum,
        }
    }

    /// Calculate the value of the function defined by the recurrence at an isolated value N.
    ///
    /// This can be done by noticing that, for k >= sqrt(N), the values of N/k are not all distinct,
    /// but fall into groups of equal values. When calculating the recurrence, these values can be
    /// grouped together, with total coefficient given by a partial sum of the a(k).
    ///
    /// Thus, to calculate g(N), we only need the distinct values g(m) for all m of the form
    /// N/k, and these values can, in turn, be calculated recursively using the same procedure.
    pub fn calculate_value_at(&self, N: u64) -> i64 {

        // Set up some space to perform the calculations in.
        let K = integer_sqrt(N);
        let M = N / K;
        let mut small_values = vec![0; M as usize + 1];
        let mut large_values = vec![0; K as usize];

        // Define the procedure to calculate the value of the function at n
        macro_rules! calculate {
            ($n:ident) => {

                // Initialise the procedure
                let switch = integer_sqrt($n);
                let mut ans = (self.f)($n);
                let (mut m, mut k) = (1, $n);

                // Perform the calculation for large k and small m
                while k >= switch {
                    let next_k = $n / (m + 1);
                    let coeff = (self.a_partial_sum)(next_k, k);
                    ans -= coeff * small_values[m as usize];
                    m = m + 1; k = next_k;
                }

                // Perform the calculation for small k and large m
                while k > 1 {
                    let m = $n / k;
                    if m <= M {
                        ans -= (self.a_partial_sum)(k - 1, k) * small_values[m as usize];
                    } else {
                        ans -= (self.a_partial_sum)(k - 1, k) * large_values[(N / m) as usize];
                    }
                    k -= 1;
                }

                // Store the computed value of g(n)
                if $n <= M {
                    small_values[$n as usize] = ans / (self.a_partial_sum)(0, 1);
                } else {
                    large_values[(N / $n) as usize] = ans / (self.a_partial_sum)(0, 1);
                }
            }
        }

        // Calculate the value we're looking for
        for m in 1..M + 1 {
            calculate!(m);
        }

        for k in (1..K).rev() {
            let m = N / k;
            calculate!(m);
        }

        if N <= M { small_values[N as usize] } else { large_values[1 as usize] }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mertens() {
        let f = |_n| 1;
        let a = |b, c| (c - b) as i64;
        let mertens_recurrence = Recurrence::new(f, a);

        assert_eq!(mertens_recurrence.calculate_value_at(100_000_000), 1928);
    }

    #[test]
    fn test_large_inputs() {
        let f_0 = |n| (n * (n + 1) / 2) as i64;
        let f_1 = |n| (n * (n + 1) * (2 * n + 1) / 6) as i64;
        let f_2 = |n| (n * n * (n + 1) * (n + 1) / 4) as i64;

        let a_0 = |b, c| (c - b) as i64;
        let a_1 = |b, c| (c * (c + 1) / 2 - b * (b + 1) / 2) as i64;
        let a_2 = |b, c| (c * (c + 1) * (2 * c + 1) / 6 - b * (b + 1) * (2 * b + 1) / 6) as i64;

        let recurrence_0 = Recurrence::new(f_0, a_0);
        let recurrence_1 = Recurrence::new(f_1, a_1);
        let recurrence_2 = Recurrence::new(f_2, a_2);

        assert_eq!(recurrence_0.calculate_value_at(1000), 304192);
        assert_eq!(recurrence_1.calculate_value_at(1000), 202870719);
        assert_eq!(recurrence_2.calculate_value_at(1000), 152210182553);
    }

    #[test]
    fn test_small_inputs() {
        let f = |n| (n * (n + 1) / 2) as i64;
        let a = |b, c| (c - b) as i64;
        let recurrence = Recurrence::new(f, a);

        assert_eq!(
            (1..11).map(|n| recurrence.calculate_value_at(n)).collect::<Vec<_>>(),
            vec![1, 2, 4, 6, 10, 12, 18, 22, 28, 32]
        );
    }
}
