//! A library for generating prime numbers using a segmented sieve.

mod iterator;
mod segsieve;
mod segment;
mod sieve;
mod wheel;

pub use sieve::{Sieve, SieveIterator};