//! Functions and iterators relating to subsets of an input set.

use search::{DepthFirstTree, Pruning};

/// An iterator over all subsets of the given input set.
///
/// # Examples
///
/// ```
/// use combinatorics::all_subsets;
///
/// let mut subsets = all_subsets(vec![1, 2, 3]);
///
/// assert_eq!(subsets.next(), Some(vec![1, 2, 3]));
/// assert_eq!(subsets.next(), Some(vec![1, 2]));
/// assert_eq!(subsets.next(), Some(vec![1, 3]));
/// assert_eq!(subsets.next(), Some(vec![1]));
/// assert_eq!(subsets.next(), Some(vec![2, 3]));
/// assert_eq!(subsets.next(), Some(vec![2]));
/// assert_eq!(subsets.next(), Some(vec![3]));
/// assert_eq!(subsets.next(), Some(vec![]));
/// assert_eq!(subsets.next(), None);
/// ```
pub fn all_subsets<T: Clone>(elements: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    SubsetsTree::new(elements).into_iter()
}

/// The state of a tree that is used to conduct a depth-first search of all subsets of an input
/// set.
struct SubsetsTree<T: Clone> {
    /// The elements of the input set
    elements: Vec<T>,
    /// The current partial subset
    subset: Vec<T>,
    /// The current depth of the search
    depth: usize,
}

/// A step that can be taken in the search tree for subsets.
type SubsetsTreeStep = bool;

impl<T: Clone> SubsetsTree<T> {

    /// Create a new `SubsetsTre` that will find subsets of the given input set.
    fn new(elements: Vec<T>) -> SubsetsTree<T> {
        let length = elements.len();
        SubsetsTree {
            elements,
            subset: Vec::with_capacity(length),
            depth: 0,
        }
    }
}

impl<T: Clone> DepthFirstTree for SubsetsTree<T> {
    type Step = SubsetsTreeStep;
    type Output = Vec<T>;

    /// Return whether or not to include the next element in the subset
    fn next_steps(&mut self) -> Vec<Self::Step> {
        vec![true, false]
    }

    /// Prune here if the fate of all elements has already been decided.
    fn should_prune(&mut self) -> Pruning {
        if self.depth == self.elements.len() {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next element (or not) to the subset
    fn apply_step(&mut self, step: &Self::Step) {
        if *step { self.subset.push(self.elements[self.depth].clone()); }
        self.depth += 1;
    }

    /// Remove the previous element (possibly) from the subset
    fn revert_step(&mut self, step: &Self::Step) {
        if *step { self.subset.pop(); }
        self.depth -= 1;
    }

    /// If all elements have been included, output the partition
    fn output(&mut self) -> Option<Self::Output> {
        if self.depth == self.elements.len() {
            Some(self.subset.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_subsets_trivial() {
        let mut it = all_subsets::<usize>(vec![]);
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_subsets_mainline() {
        let mut it = all_subsets(vec![1, 2, 3]);
        assert_eq!(it.next(), Some(vec![1, 2, 3]));
        assert_eq!(it.next(), Some(vec![1, 2]));
        assert_eq!(it.next(), Some(vec![1, 3]));
        assert_eq!(it.next(), Some(vec![1]));
        assert_eq!(it.next(), Some(vec![2, 3]));
        assert_eq!(it.next(), Some(vec![2]));
        assert_eq!(it.next(), Some(vec![3]));
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }
}
