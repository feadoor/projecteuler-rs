//! A collection of utilities for working with generators

#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

/// An adapter that allows a generator to be used as an iterator.
///
/// # Examples
///
/// ```
/// #![feature(generators)]
/// use generators::GeneratorIteratorAdapter;
///
/// let range = || for i in 0..3 { yield i; };
/// let mut range_iter = GeneratorIteratorAdapter::of(range);
///
/// assert_eq!(range_iter.next(), Some(0));
/// assert_eq!(range_iter.next(), Some(1));
/// assert_eq!(range_iter.next(), Some(2));
/// assert_eq!(range_iter.next(), None);
/// ```
pub struct GeneratorIteratorAdapter<G: Generator<Return = ()>>(G);

impl <G: Generator<Return = ()>> GeneratorIteratorAdapter<G> {
    pub fn of(generator: G) -> GeneratorIteratorAdapter<G> {
        GeneratorIteratorAdapter(generator)
    }
}

impl <G: Generator<Return = ()>> Iterator for GeneratorIteratorAdapter<G> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { self.0.resume() } {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

/// A macro that yields all values from one generator within another generator.
///
/// # Examples
///
/// ```
/// #![feature(generators, generator_trait)]
/// use generators::{GeneratorIteratorAdapter, yield_from};
/// use std::ops::{Generator, GeneratorState};
///
/// let range1 = || for i in 0..2 { yield i; };
/// let range2 = || for i in 2..4 { yield i; };
///
/// let combined = move || { yield_from!(range1); yield_from!(range2); };
/// let mut combined_iter = GeneratorIteratorAdapter::of(combined);
///
/// assert_eq!(combined_iter.next(), Some(0));
/// assert_eq!(combined_iter.next(), Some(1));
/// assert_eq!(combined_iter.next(), Some(2));
/// assert_eq!(combined_iter.next(), Some(3));
/// assert_eq!(combined_iter.next(), None);
/// ```
#[macro_export]
macro_rules! yield_from {
    ($g:expr) => {
        let mut generator = $g;
        loop {
            match unsafe { generator.resume() } {
                GeneratorState::Yielded(x) => { yield x; },
                GeneratorState::Complete(_) => { break; }
            }
        }
    }
}
