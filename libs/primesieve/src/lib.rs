//! A library for generating prime numbers using a segmented sieve.

extern crate modular_arithmetic;
extern crate number_theory;

mod iterator;
mod segsieve;
mod segment;
mod sieve;
mod wheel;

pub use sieve::{Sieve, SieveIterator};