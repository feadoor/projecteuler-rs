//! Functions, such as factorisation and similar computations, which require use of prime numbers
//! to be calculated.

use segment;
use sieve::Sieve;

impl Sieve {
    /// Uses trial division to determine if the given number is prime.
    fn trial_division(&self, n: u64) -> bool {
        for p in self.iter() {
            if p.saturating_mul(p) > n {
                return true;
            } else if n % p == 0 {
                return false;
            }
        }

        true
    }

    /// Returns whether or not `n` is a prime number, or `Err(())` if `n` is larger than the square
    /// of the largest prime held in the sieve.
    ///
    /// Uses a simple lookup if `n` is not greater than the largest number known about by the
    /// sieve, and uses trial division otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.is_prime(0), Ok(false));
    /// assert_eq!(sieve.is_prime(1), Ok(false));
    /// assert_eq!(sieve.is_prime(2), Ok(true));
    /// assert_eq!(sieve.is_prime(3), Ok(true));
    /// assert_eq!(sieve.is_prime(4), Ok(false));
    /// assert_eq!(sieve.is_prime(5), Ok(true));
    ///
    /// assert_eq!(sieve.is_prime(491), Ok(true));
    /// assert_eq!(sieve.is_prime(493), Ok(false));
    /// assert_eq!(sieve.is_prime(495), Ok(false));
    /// assert_eq!(sieve.is_prime(497), Ok(false));
    /// assert_eq!(sieve.is_prime(499), Ok(true));
    ///
    /// assert_eq!(sieve.is_prime(1000001), Err(()));
    /// ```
    pub fn is_prime(&self, n: u64) -> Result<bool, ()> {
        match n {
            2 | 3 | 5 => Ok(true),
            _ => {
                if n < self.limit() {
                    Ok(segment::get(&self.primes, n))
                } else if n <= self.limit().saturating_mul(self.limit()) {
                    Ok(Sieve::trial_division(self, n))
                } else {
                    Err(())
                }
            }
        }
    }

    /// Factorises `n` into (prime, exponent) pairs.
    ///
    /// Returns `Err(remainder, partial factorisation)` if `n` cannot be fully factorised without
    /// sieving for more primes.
    ///
    /// If `x` is the largest number known about by the sieve, then any integer having at most one
    /// prime factor larger than `x` can be factorised. In particular, any number not greater than
    /// `x^2` can be factorised.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.factorise(2), Ok(vec![(2, 1)]));
    /// assert_eq!(sieve.factorise(4), Ok(vec![(2, 2)]));
    /// assert_eq!(sieve.factorise(1 << 63), Ok(vec![(2, 63)]));
    ///
    /// assert_eq!(sieve.factorise(2 * 3), Ok(vec![(2, 1), (3, 1)]));
    /// assert_eq!(sieve.factorise(89 * 97), Ok(vec![(89, 1), (97, 1)]));
    /// assert_eq!(sieve.factorise(8 * 9 * 5), Ok(vec![(2, 3), (3, 2), (5, 1)]));
    ///
    /// assert_eq!(sieve.factorise(2 * 3 * 5 * 991), Ok(vec![(2, 1), (3, 1), (5, 1), (991, 1)]));
    /// assert_eq!(sieve.factorise(2 * 3 * 5 * 991 * 991),
    ///            Err((991 * 991, vec![(2, 1), (3, 1), (5, 1)])));
    /// ```
    pub fn factorise(&self, mut n: u64) -> Result<Vec<(u64, u64)>, (u64, Vec<(u64, u64)>)> {
        // Deal with small values of `n` as special cases.
        if n == 0 { return Err((0, vec![])) }
        if n == 1 { return Ok(vec![]) }

        // Somewhere to store the result.
        let mut factors = Vec::new();

        // Iterate over the primes held in the sieve, checking if they are divisors of `n`.
        for p in self.iter() {

            // Check if `p` is large enough that we can stop iterating.
            if p.saturating_mul(p) > n {
                break;
            }

            // Repeatedly divide `n` by `p` until it is no longer divisible.
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                factors.push((p, count));
            }
        }

        // If there are any leftovers, check if it is small enough that we can guarantee that it
        // is prime.
        if n != 1 {
            if self.limit().saturating_mul(self.limit()) < n {
                return Err((n, factors));
            } else {
                factors.push((n, 1));
            }
        }

        Ok(factors)
    }

    /// Calculates the value of Euler's totient function `ϕ` at `n`.
    ///
    /// Uses the formula based on the factorisation of `n`, that is `ϕ(n)` is equal to `n` times
    /// the product of `1 - 1/p`, where `p` ranges over the distinct prime factors of `n`.
    ///
    /// Returns `Err(())` if `n` cannot be factorised without first sieving for more primes.
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.euler_phi(2), Ok(1));
    /// assert_eq!(sieve.euler_phi(4), Ok(2));
    /// assert_eq!(sieve.euler_phi(1 << 63), Ok(1 << 62));
    ///
    /// assert_eq!(sieve.euler_phi(2 * 3), Ok(2));
    /// assert_eq!(sieve.euler_phi(89 * 97), Ok(88 * 96));
    /// assert_eq!(sieve.euler_phi(8 * 9 * 5), Ok(4 * 6 * 4));
    ///
    /// assert_eq!(sieve.euler_phi(2 * 3 * 5 * 991), Ok(2 * 4 * 990));
    /// assert_eq!(sieve.euler_phi(2 * 3 * 5 * 991 * 991), Err(()));
    /// ```
    pub fn euler_phi(&self, mut n: u64) -> Result<u64, ()> {
        if let Ok(factors) = self.factorise(n) {
            for (p, _) in factors {
                n = (n / p) * (p - 1);
            }
            Ok(n)
        } else {
            Err(())
        }
    }

    /// Calculates the number of divisors of `n`.
    ///
    /// Returns Err(()) is `n` cannot be fully factorised without first sieving for more primes.
    ///
    /// This uses the well-known formula, that if `n` is given in factorised form as a product
    /// `p_i ^ a_i`, then the number of divisors of `n` is given by:
    ///
    /// (a_1 + 1)(a_2 + 1) ... (a_k + 1)
    ///
    /// # Examples
    ///
    /// ```
    /// let sieve = primesieve::Sieve::to_limit(100);
    ///
    /// assert_eq!(sieve.number_of_divisors(2), Ok(2));
    /// assert_eq!(sieve.number_of_divisors(4), Ok(3));
    /// assert_eq!(sieve.number_of_divisors(1 << 63), Ok(64));
    ///
    /// assert_eq!(sieve.number_of_divisors(2 * 3), Ok(2 * 2));
    /// assert_eq!(sieve.number_of_divisors(89 * 97), Ok(2 * 2));
    /// assert_eq!(sieve.number_of_divisors(8 * 9 * 5), Ok(4 * 3 * 2));
    ///
    /// assert_eq!(sieve.number_of_divisors(2 * 3 * 5 * 991), Ok(2 * 2 * 2 * 2));
    /// assert_eq!(sieve.number_of_divisors(2 * 3 * 5 * 991 * 991), Err(()));
    /// ```
    pub fn number_of_divisors(&self, n: u64) -> Result<u64, ()> {
        if let Ok(factors) = self.factorise(n) {
            Ok(factors.iter().map(|x| x.1 + 1).product())
        } else {
            Err(())
        }
    }
}
