//! An iterator adaptor that provides all permutations of the elements of the input iterator, taking
//! into account that there may be repeated elements in the input iterator.

/// An iterator through the permutations of an input iterator, taking into account that elements
/// may be equal.
pub struct Permutations<I: Iterator> {
    items: Vec<I::Item>,
    first: bool,
}

/// Create a new `Permutations` from an input iterator
pub fn permutations<I: Iterator>(iterator: I) -> Permutations<I> where I::Item: Ord {
    let mut items: Vec<_> = iterator.collect(); items.sort();
    Permutations {
        items: items,
        first: true,
    }
}

impl<I: Iterator> Iterator for Permutations<I> where I::Item: Clone + Ord {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first { self.first = false; }
        else if self.items.len() == 0 { return None; }
        else {

            // Find the smallest `j` such that all permutations beginning with `a₁, ... , aⱼ` have
            // already been visited.
            let mut j = self.items.len() - 2;
            while j > 0 && self.items[j] >= self.items[j + 1] { j -= 1; }
            if self.items[j] >= self.items[j + 1] { return None; }

            // Find the index `k` so that `aₖ` goes immediately to the right of `aⱼ` and swap them
            let mut k = self.items.len() - 1;
            while self.items[j] >= self.items[k] { k -= 1; }
            self.items.swap(j, k);

            // Reverse the suffix of the resulting permutation
            self.items[j + 1..].reverse()
        }

        Some(self.items.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_trivial_permutations() {
        let mut it = permutations(0..0);
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mainline_permutations() {
        let mut it = permutations(1..4);
        assert_eq!(it.next(), Some(vec![1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 3, 2]));
        assert_eq!(it.next(), Some(vec![2, 1, 3]));
        assert_eq!(it.next(), Some(vec![2, 3, 1]));
        assert_eq!(it.next(), Some(vec![3, 1, 2]));
        assert_eq!(it.next(), Some(vec![3, 2, 1]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_permutations_with_repeats() {
        let numbers = vec![1, 1, 2, 3];
        let mut it = permutations(numbers.iter().map(|x| *x));
        assert_eq!(it.next(), Some(vec![1, 1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 1, 3, 2]));
        assert_eq!(it.next(), Some(vec![1, 2, 1, 3]));
        assert_eq!(it.next(), Some(vec![1, 2, 3, 1]));
        assert_eq!(it.next(), Some(vec![1, 3, 1, 2]));
        assert_eq!(it.next(), Some(vec![1, 3, 2, 1]));
        assert_eq!(it.next(), Some(vec![2, 1, 1, 3]));
        assert_eq!(it.next(), Some(vec![2, 1, 3, 1]));
        assert_eq!(it.next(), Some(vec![2, 3, 1, 1]));
        assert_eq!(it.next(), Some(vec![3, 1, 1, 2]));
        assert_eq!(it.next(), Some(vec![3, 1, 2, 1]));
        assert_eq!(it.next(), Some(vec![3, 2, 1, 1]));
        assert_eq!(it.next(), None);
    }
}
