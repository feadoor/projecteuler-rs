//! An iterator adaptor that provides combinations of the elements from an input iterator, with
//! replacement.

use crate::lazy_buffer::LazyBuffer;

/// An iterator through the length `n` combinations of an input iterator, allowing elements to be
/// repeated within each combination.
pub struct CombinationsWithReplacement<I: Iterator> {
    n: usize,
    indices: Vec<usize>,
    items: LazyBuffer<I>,
    first: bool,
}

/// Create a new `CombinationsWithReplacement` from an input iterator
pub fn combinations_with_replacement<I: Iterator>(iterator: I, n: usize) -> CombinationsWithReplacement<I> {

    let mut items = LazyBuffer::new(iterator);
    items.get_next();

    CombinationsWithReplacement {
        n: n,
        indices: vec![0; n],
        items: items,
        first: true,
    }
}

impl<I: Iterator> Iterator for CombinationsWithReplacement<I> where I::Item: Clone {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut items_len = self.items.len();
        if self.items.is_done() && items_len == 0 {
            return None;
        }

        if self.first { self.first = false; }
        else if self.n == 0 { return None; }
        else {

            // Check if we need to consume more items from the input iterator
            if self.indices[self.n - 1] == items_len - 1 && !self.items.is_done() {
                if self.items.get_next() {
                    items_len += 1;
                }
            }

            // Look for the first index from the end that can be incremented
            let increment = (0..self.n).rev().find(|&i| self.indices[i] != items_len - 1);
            match increment {
                Some(idx) => {
                    self.indices[idx] += 1;
                    for jdx in idx + 1..self.n { self.indices[jdx] = self.indices[idx]; }
                },
                None => { return None; },
            }
        }

        // Produce a result vector based on the indices
        let mut result = Vec::with_capacity(self.n);
        for idx in self.indices.iter() {
            result.push(self.items[*idx].clone());
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_trivial_cwr() {
        let mut it = combinations_with_replacement(0..0, 3);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_zero_length_cwr() {
        let mut it = combinations_with_replacement(1..4, 0);
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mainline_cwr() {
        let mut it = combinations_with_replacement(1..5, 3);
        assert_eq!(it.next(), Some(vec![1, 1, 1]));
        assert_eq!(it.next(), Some(vec![1, 1, 2]));
        assert_eq!(it.next(), Some(vec![1, 1, 3]));
        assert_eq!(it.next(), Some(vec![1, 1, 4]));
        assert_eq!(it.next(), Some(vec![1, 2, 2]));
        assert_eq!(it.next(), Some(vec![1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 2, 4]));
        assert_eq!(it.next(), Some(vec![1, 3, 3]));
        assert_eq!(it.next(), Some(vec![1, 3, 4]));
        assert_eq!(it.next(), Some(vec![1, 4, 4]));
        assert_eq!(it.next(), Some(vec![2, 2, 2]));
        assert_eq!(it.next(), Some(vec![2, 2, 3]));
        assert_eq!(it.next(), Some(vec![2, 2, 4]));
        assert_eq!(it.next(), Some(vec![2, 3, 3]));
        assert_eq!(it.next(), Some(vec![2, 3, 4]));
        assert_eq!(it.next(), Some(vec![2, 4, 4]));
        assert_eq!(it.next(), Some(vec![3, 3, 3]));
        assert_eq!(it.next(), Some(vec![3, 3, 4]));
        assert_eq!(it.next(), Some(vec![3, 4, 4]));
        assert_eq!(it.next(), Some(vec![4, 4, 4]));
        assert_eq!(it.next(), None);
    }
}
