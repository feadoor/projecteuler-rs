//! [Problem 78 (Coin partitions)](https://projecteuler.net/problem=78)
//!
//! # Solution detail
//!
//! Simply generate the sequence of partition numbers, modulo 1,000,000, until the first term
//! which is divisible by 1,000,000 is found.
//!
//! Use Euler's [Pentagonal number theorem](https://en.wikipedia.org/wiki/Pentagonal_number_theorem#Relation_with_partitions)
//! to efficiently generate the sequence of partition numbers with respect to a given modulus.

#[macro_use]
extern crate projecteuler_rs;
#[macro_use]
extern crate modular_arithmetic;
extern crate number_theory;

use modular_arithmetic::{Modulus, FixedModular};
use number_theory::partition_numbers;

const MODULUS: u64 = 1_000_000;
define_modulus!(ModInt, Mod, MODULUS);

/// Find the index of the first partition number divisible by `MODULUS`
fn solve() -> usize {
    partition_numbers::<ModInt>().position(|x| x.value == 0).unwrap()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve().to_string()
}

problem!(answer, "55374");
