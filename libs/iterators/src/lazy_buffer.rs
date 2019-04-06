//! A structure that allows indexing into an iterator, lazily evaluating as many elements of the
//! iterator as necessary, but no more.

use std::ops::Index;

pub struct LazyBuffer<I: Iterator> {
    iterator: I,
    done: bool,
    items: Vec<I::Item>,
}

impl<I: Iterator> LazyBuffer<I> {

    pub fn new(iterator: I) -> LazyBuffer<I> {
        LazyBuffer {
            iterator: iterator,
            done: false,
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn get_next(&mut self) -> bool {
        if self.done { return false; }
        match self.iterator.next() {
            Some(item) => {
                self.items.push(item);
                true
            },
            None => {
                self.done = true;
                false
            }
        }
    }
}

impl<I: Iterator> Index<usize> for LazyBuffer<I> {
    type Output = I::Item;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        self.items.index(idx)
    }
}
