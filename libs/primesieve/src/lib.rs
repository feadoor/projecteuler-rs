//! A library for generating prime numbers using a segmented sieve.

extern crate num_traits;
extern crate number_theory;

mod iterator;
mod segsieve;
mod segment;
mod sieve;
mod wheel;

pub use sieve::{Sieve, SieveIterator};