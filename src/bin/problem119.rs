//! [Problem 119 (Digit power sum)](https://projecteuler.net/problem=119)
//!
//! # Solution detail
//!
//! Given a perfect power, it is easy to check whether it is a power of the sum of its digits - 
//! just add up the digits and see!
//!
//! We can solve this problem, therefore, by iterating over all perfect powers in increasing order.
//! This can be done by using a priority queue. For each power, store its base, its exponent and
//! its value, and pop them off the priority queue in order of value.
//!
//! Each time a power is popped, store in the queue the next power up with the same base.
//! Additionally, keep track of the first base not yet included in the queue and make sure to add
//! it at an opportune moment - i.e. when we first include a power whose value exceeds the power
//! with the new base.
//!
//! To avoid storing useless powers that can never be the solution, we can also calculate a rough
//! lower bound on the smallest exponent to consider for each base. For example, we never need to
//! consider if the square of a 3 digit number will be a solution - because the square will have
//! at most 6 digits, the sum of which will be at most 54.
//!
//! In general, for a base `a` with `d` digits, `aᵏ` will have at most `kd` digits, meaning that we
//! only need to consider exponents `k` for which `9kd > 10ᵈ⁻¹`, in other words `k > 10ᵈ⁻¹ / 9d`.

use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;

use number_theory::{pow, checked_pow};
use projecteuler_rs::problem;

/// A structure that represents a perfect power
#[derive(Copy, Clone, Debug)]
struct Power {
    base: u64,
    exponent: u64,
    value: u64,
}

impl Power {

    /// Construct a new `Power` with the given base and exponent
    fn from(base: u64, exponent: u64) -> Option<Power> {
        checked_pow(base, exponent).map(|value| Power { base, exponent, value})
    }

    /// Increase the exponent of this power by one and produce a new power
    fn with_next_exponent(&self) -> Option<Power> {
        self.value.checked_mul(self.base).map(|value| {
            Power { base: self.base, exponent: self.exponent + 1, value: value }
        })
    }

    /// Check if this is a digit sum power
    fn is_digit_sum(&self) -> bool {
        sum_of_digits(self.value) == self.base
    }
}

// Implement required traits for `Power` that allow it to be stored in a `BinaryHeap`

impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Power {}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value).map(|x| x.reverse())
    }
}

impl Ord for Power {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


/// An iterator over sequential perfect powers, ignoring those powers which can never be a digit
/// sum perfect power.
struct PowersIterator {
    /// A priority queue where perfect powers are queued up in order of value
    queue: BinaryHeap<Power>,
    /// The first power of the "next" base that hasn't yet been added to the queue
    next_base: Option<Power>,
}

impl PowersIterator {

    /// Construct a new `PowersIterator` that will iterate over useful perfect powers
    fn new() -> PowersIterator {
        let mut queue = BinaryHeap::new(); queue.push(Power::from(2, 2).unwrap());
        PowersIterator {
            queue,
            next_base: Power::from(3, 2),
        }
    }

    /// Enqueue the given `Power` by adding it the the `BinaryHeap`
    fn enqueue(&mut self, power: Option<Power>) {
        if let Some(p) = power {

            // Add this new power to the queue
            self.queue.push(p);

            // Check if we should also add the first power of the next base to the queue
            if let Some(base) = self.next_base {
                if p.value >= base.value {
                    self.queue.push(base);
                    self.next_base = Power::from(base.base + 1, min_viable_exponent(base.base + 1));
                }
            }
        }
    }
}

impl Iterator for PowersIterator {
    type Item = Power;

    /// Get the next power from the queue and add the same power with an increased exponent
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|p| {
            self.enqueue(p.with_next_exponent());
            p
        })
    }
}

/// Returns the sum of the digits in the given number
fn sum_of_digits(mut number: u64) -> u64 {
    let mut result = 0;
    while number > 0 { result += number % 10; number /= 10; }
    result
}

/// Returns the number of digits in the given number
fn num_digits(mut number: u64) -> u64 {
    let mut digits = 0;
    while number > 0 { number /= 10; digits += 1; }
    digits
}

/// Returns the smallest exponent which, for this base, could form a digit-sum power.
fn min_viable_exponent(base: u64) -> u64 {
    let digits = num_digits(base);
    max(2, pow(10, digits - 1) / (9 * digits) + 1)
}

/// Find the nth number which is the sum of its digits raised to some perfect power.
fn solve(n: usize) -> u64 {
    PowersIterator::new().filter(|&p| p.is_digit_sum()).map(|p| p.value).nth(n - 1).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(30).to_string()
}

problem!(answer, "248155780267521");
