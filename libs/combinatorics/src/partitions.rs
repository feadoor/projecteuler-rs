//! Functions and iterators relating to partitions of an input set.

use search::{DepthFirstTree, Pruning};

/// An iterator over all partitions of the given input set.
///
/// # Examples
///
/// ```
/// use combinatorics::all_partitions;
///
/// let mut partitions = all_partitions(vec![1, 2, 3]);
///
/// assert_eq!(partitions.next(), Some(vec![vec![1, 2, 3]]));
/// assert_eq!(partitions.next(), Some(vec![vec![1, 2], vec![3]]));
/// assert_eq!(partitions.next(), Some(vec![vec![1, 3], vec![2]]));
/// assert_eq!(partitions.next(), Some(vec![vec![1], vec![2, 3]]));
/// assert_eq!(partitions.next(), Some(vec![vec![1], vec![2], vec![3]]));
/// assert_eq!(partitions.next(), None);
/// ```
pub fn all_partitions<T: Clone>(elements: Vec<T>) -> impl Iterator<Item = Vec<Vec<T>>> {
    PartitionsTree::new(elements).into_iter()
}

/// The state of a tree that is used to conduct a depth-first search of all partitions of an input
/// set.
struct PartitionsTree<T: Clone> {
    /// The elements of the input set
    elements: Vec<T>,
    /// The current partial partition
    partition: Vec<Vec<T>>,
    /// The current depth of the search
    depth: usize,
}

/// A step that can be taken in the search tree for partitions.
struct PartitionsTreeStep<T: Clone> {
    /// The index of the set within the partition to add the element to
    index: usize,
    /// The value of the element to add
    value: T,
}

impl<T: Clone> PartitionsTree<T> {

    /// Create a new `PartitionsTree` that will find partitions of the given input set.
    fn new(elements: Vec<T>) -> PartitionsTree<T> {
        let length = elements.len();
        PartitionsTree {
            elements,
            partition: Vec::with_capacity(length),
            depth: 0,
        }
    }
}

impl<T: Clone> DepthFirstTree for PartitionsTree<T> {
    type Step = PartitionsTreeStep<T>;
    type Output = Vec<Vec<T>>;

    /// Return all possible choices for the position of the next element in the partition
    fn next_steps(&mut self) -> Vec<Self::Step> {
        (0..self.partition.len() + 1).map(|index|
            PartitionsTreeStep { index, value: self.elements[self.depth].clone()
        }).collect()
    }

    /// Prune here if all elements have already been added to the partition.
    fn should_prune(&mut self) -> Pruning {
        if self.depth == self.elements.len() {
            Pruning::Below
        } else {
            Pruning::None
        }
    }

    /// Add the next element to the partition
    fn apply_step(&mut self, step: &Self::Step) {
        if step.index == self.partition.len() {
            self.partition.push(vec![step.value.clone()]);
        } else {
            self.partition[step.index].push(step.value.clone());
        }
        self.depth += 1;
    }

    /// Remove the previous element from the partition
    fn revert_step(&mut self, step: &Self::Step) {
        if step.index == self.partition.len() - 1 {
            if self.partition[step.index].len() == 1 {
                self.partition.pop();
            } else {
                self.partition[step.index].pop();
            };
        } else {
            self.partition[step.index].pop();
        }
        self.depth -= 1;
    }

    /// If all elements have been included, output the partition
    fn output(&mut self) -> Option<Self::Output> {
        if self.depth == self.elements.len() {
            Some(self.partition.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_partitions_trivial() {
        let mut it = all_partitions::<usize>(vec![]);
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_partitions_mainline() {
        let mut it = all_partitions(vec![1, 2, 3]);
        assert_eq!(it.next(), Some(vec![vec![1, 2, 3]]));
        assert_eq!(it.next(), Some(vec![vec![1, 2], vec![3]]));
        assert_eq!(it.next(), Some(vec![vec![1, 3], vec![2]]));
        assert_eq!(it.next(), Some(vec![vec![1], vec![2, 3]]));
        assert_eq!(it.next(), Some(vec![vec![1], vec![2], vec![3]]));
        assert_eq!(it.next(), None);
    }
}
