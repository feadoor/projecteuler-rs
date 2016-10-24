//! A library for generating prime numbers using a segmented sieve.

extern crate num_traits;

mod iterator;
mod segsieve;
mod segment;
mod sieve;
mod wheel;

pub use sieve::{Sieve, SieveIterator};