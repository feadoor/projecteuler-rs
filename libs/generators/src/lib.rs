//! A collection of utilities for working with generators

#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};

/// An adapter that allows a generator to be used as an iterator.
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

#[macro_export]
macro_rules! yield_from {
    ($g:expr) => {
        loop {
            match unsafe { $g.resume() } {
                GeneratorState::Yielded(x) => { yield x; },
                GeneratorState::Complete(_) => { break; }
            }
        }
    }
}
