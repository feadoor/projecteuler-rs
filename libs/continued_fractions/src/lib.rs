//! A library relating to continued fraction expansions, particularly of square roots of integers.

use num::{BigUint, Zero, One};
use number_theory::integer_sqrt;
use std::mem::replace;

/// A structure representing a periodic continued fraction.
pub struct PeriodicContinuedFraction {
    /// The terms in the tail of this continued fraction.
    pub tail: Vec<u64>,
    /// The terms in the period of this continued fraction.
    pub period: Vec<u64>,
}

impl PeriodicContinuedFraction {

    /// Create a `PeriodicContinuedFraction` with the given tail and period.
    pub fn new(tail: Vec<u64>, period: Vec<u64>) -> PeriodicContinuedFraction {
        PeriodicContinuedFraction {
            tail,
            period,
        }
    }

    /// Compute a `PeriodicContinuedFraction` for the square root of the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// use continued_fractions::PeriodicContinuedFraction;
    ///
    /// let sqrt3 = PeriodicContinuedFraction::sqrt(3);
    /// assert_eq!(sqrt3.tail, vec![1]);
    /// assert_eq!(sqrt3.period, vec![1, 2]);
    /// ```
    pub fn sqrt(n: u64) -> PeriodicContinuedFraction {
        let tail = integer_sqrt(n);
        let mut period = Vec::new();
        if tail * tail != n {
            let (mut a, mut p, mut q) = (tail, 0, 1);
            loop {
                p = a * q - p;
                q = (n - p * p) / q;
                a = (tail + p) / q;
                period.push(a);

                if q == 1 {
                    break;
                }
            }
        }

        PeriodicContinuedFraction {
            tail: vec![tail],
            period,
        }
    }
}

impl<'a> PeriodicContinuedFraction {
    /// Return an iterator over the terms in this continued fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use continued_fractions::PeriodicContinuedFraction;
    ///
    /// let cf = PeriodicContinuedFraction::new(vec![1, 2], vec![3, 4, 5]);
    /// assert_eq!(cf.iter().take(10).collect::<Vec<u64>>(),
    ///            vec![1, 2, 3, 4, 5, 3, 4, 5, 3, 4]);
    /// ```
    pub fn iter(&'a self) -> PeriodicContinuedFractionIterator<'a> {
        let first_tail_idx = if self.tail.len() == 0 {
            None
        } else {
            Some(0)
        };

        PeriodicContinuedFractionIterator {
            next_tail_idx: first_tail_idx,
            next_period_idx: 0,
            continued_fraction: self,
        }
    }

    /// Return an iterator over the convergents of this continued fraction.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use continued_fractions::PeriodicContinuedFraction;
    ///
    /// let cf = PeriodicContinuedFraction::new(vec![1], vec![2]);
    /// let mut convergents = cf.convergents().map(|(numer, denom)| (numer.clone(), denom.clone()));
    ///
    /// assert_eq!(convergents.next(), Some((From::from(1u64), From::from(1u64))));
    /// assert_eq!(convergents.next(), Some((From::from(3u64), From::from(2u64))));
    /// assert_eq!(convergents.next(), Some((From::from(7u64), From::from(5u64))));
    /// assert_eq!(convergents.next(), Some((From::from(17u64), From::from(12u64))));
    /// assert_eq!(convergents.next(), Some((From::from(41u64), From::from(29u64))));
    /// assert_eq!(convergents.next(), Some((From::from(99u64), From::from(70u64))));
    /// ```
    pub fn convergents(&'a self) -> ContinuedFractionConvergents<PeriodicContinuedFractionIterator<'a>> {
        ContinuedFractionConvergents::new(self.iter())
    }
}

/// A structure capable of iterating over the terms in a periodic continued fraction.
pub struct PeriodicContinuedFractionIterator<'a> {
    /// The index of the next tail term to produce.
    next_tail_idx: Option<usize>,
    /// The index of the next periodic term to produce.
    next_period_idx: usize,
    /// The structure holding the actual terms in the continued fraction.
    continued_fraction: &'a PeriodicContinuedFraction,
}

impl<'a> Iterator for PeriodicContinuedFractionIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.next_tail_idx {
            Some(idx) => {
                self.next_tail_idx = if idx + 1 < self.continued_fraction.tail.len() {
                    Some(idx + 1)
                } else {
                    None
                };
                Some(self.continued_fraction.tail[idx])
            },
            None => {
                if self.continued_fraction.period.len() == 0 {
                    None
                } else {
                    let idx = self.next_period_idx;
                    self.next_period_idx = if idx + 1 < self.continued_fraction.period.len() {
                        idx + 1
                    } else {
                        0
                    };
                    Some(self.continued_fraction.period[idx])
                }
            }
        }
    }
}

/// A structure for iterating over the convergents resulting from the given continued fraction.
pub struct ContinuedFractionConvergents<I: Iterator<Item = u64>> {
    /// The numerators of the previous two convergents.
    numers: (BigUint, BigUint),
    /// The denominators of the previous two convergents.
    denoms: (BigUint, BigUint),
    /// The terms in the continued fraction expansion.
    terms: I
}

impl<I: Iterator<Item = u64>> ContinuedFractionConvergents<I> {
    /// Create a `ContinuedFractionConvergents` using the items from the given iterator as the terms
    /// in the continued fraction expansion.
    pub fn new(terms: I) -> ContinuedFractionConvergents<I> {
        ContinuedFractionConvergents {
            numers: (BigUint::zero(), BigUint::one()),
            denoms: (BigUint::one(), BigUint::zero()),
            terms: terms,
        }
    }
}

impl<I: Iterator<Item = u64>> Iterator for ContinuedFractionConvergents<I> {
    type Item = (BigUint, BigUint);

    fn next(&mut self) -> Option<(BigUint, BigUint)> {
        match self.terms.next() {
            Some(a) => {
                let next_numer = a * &self.numers.1 + &self.numers.0;
                let next_denom = a * &self.denoms.1 + &self.denoms.0;

                replace(&mut self.numers.0, replace(&mut self.numers.1, next_numer.clone()));
                replace(&mut self.denoms.0, replace(&mut self.denoms.1, next_denom.clone()));

                Some((next_numer, next_denom))
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        const TEST_CASES: &'static [(u64, &'static [u64], &'static [u64])] = &[
            (2, &[1], &[2]),
            (3, &[1], &[1, 2]),
            (5, &[2], &[4]),
            (6, &[2], &[2, 4]),
            (7, &[2], &[1, 1, 1, 4]),
            (8, &[2], &[1, 4]),
            (139, &[11], &[1, 3, 1, 3, 7, 1, 1, 2, 11, 2, 1, 1, 7, 3, 1, 3, 1, 22]),
        ];

        for &(n, tail, period) in TEST_CASES {
            let sqrtn = PeriodicContinuedFraction::sqrt(n);
            assert_eq!(sqrtn.tail, tail);
            assert_eq!(sqrtn.period, period);
        }
    }

    #[test]
    fn test_iterator() {
        const TEST_CASES: &'static [(&'static [u64], &'static [u64], &'static [u64])] = &[
          (&[], &[], &[]),
          (&[], &[1], &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
          (&[], &[1, 2, 3], &[1, 2, 3, 1, 2, 3, 1, 2, 3, 1]),
          (&[1], &[], &[1]),
          (&[1, 2, 3], &[], &[1, 2, 3]),
          (&[1], &[2], &[1, 2, 2, 2, 2, 2, 2, 2, 2, 2]),
          (&[1, 2, 3], &[4, 5, 6], &[1, 2, 3, 4, 5, 6, 4, 5, 6, 4]),
        ];

        for &(tail, period, terms) in TEST_CASES {
            let cf = PeriodicContinuedFraction::new(tail.to_vec(), period.to_vec());
            assert_eq!(cf.iter().take(10).collect::<Vec<u64>>(), terms);
        }
    }

    #[test]
    fn test_convergents() {
        const TEST_CASES: &'static [(&'static [u64], &'static [(u64, u64)])] = &[
            (&[1, 2, 2, 2, 2, 2], &[(1, 1), (3, 2), (7, 5), (17, 12), (41, 29), (99, 70)]),
            (&[1, 1, 2, 1, 2, 1], &[(1, 1), (2, 1), (5, 3), (7, 4), (19, 11), (26, 15)]),
            (&[2, 4, 4, 4, 4, 4], &[(2, 1), (9, 4), (38, 17), (161, 72), (682, 305), (2889, 1292)]),
            (&[2, 2, 4, 2, 4, 2], &[(2, 1), (5, 2), (22, 9), (49, 20), (218, 89), (485, 198)]),
            (&[2, 1, 1, 1, 4, 1], &[(2, 1), (3, 1), (5, 2), (8, 3), (37, 14), (45, 17)]),
            (&[2, 1, 4, 1, 4, 1], &[(2, 1), (3, 1), (14, 5), (17, 6), (82, 29), (99, 35)]),
        ];

        for &(terms, expected_convergents) in TEST_CASES {
            let actual_convergents = ContinuedFractionConvergents::new(terms.iter().map(|x| *x));
            assert_eq!(actual_convergents.collect::<Vec<_>>(),
                       expected_convergents.iter().map(|&(x, y)| (From::from(x), From::from(y))).collect::<Vec<_>>());
        }
    }
}
