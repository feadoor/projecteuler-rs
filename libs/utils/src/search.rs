//! Implementations of useful utility functions related to searching.

use std::marker::Sized;

/// Finds the smallest value of `n` for which func(n) is at least the target value using a binary
/// search. Assumes that such a value exists, and that the function is increasing.
///
/// # Examples
///
/// ```
/// use utils::search::binary_search;
///
/// assert_eq!(binary_search(&|n| n + 17, 22), 5);
/// assert_eq!(binary_search(&|n| n * n, 2000), 45);
/// ```
pub fn binary_search<F>(func: &F, target: u64) -> u64
    where F: Fn(u64) -> u64
{
    // Try to find an initial range that the solution lies in.
    let mut lower = 0;
    let mut upper = 1;
    while func(upper) < target {
        lower = upper;
        upper *= 2;
    }

    // Use binary search to find the desired value.
    while lower != upper {
        let mid = (lower + upper - 1) / 2;
        if func(mid) < target {
            lower = mid + 1;
        } else {
            upper = mid;
        }
    }

    lower
}

/// A trait for nodes in a tree which is to be traversed, depth-first, in the pursuit of nodes which
/// satisfy a particular condition.
pub trait DepthFirstNode where Self: Sized {
    /// Returns all the nodes which are direct descendants of this node.
    fn children(&self) -> Vec<Self>;
    /// Returns `true` if the search tree can be pruned below this node - that is, none of its
    /// children can possibly satisfy the condition.
    fn should_prune(&self) -> bool;
    /// Returns `true` if this node satisfies the condition.
    fn accept(&self) -> bool;
}

/// A structure which is used for iterating through a tree, depth-first, producing only those nodes
/// which satisfy a particular condition.
///
/// # Examples
///
/// ```
/// use utils::search::{DepthFirstSearcher, DepthFirstNode};
///
/// // Find all numbers below 1000 with only odd digits and divisible by 13.
/// struct Node {
///     value: u32,
/// }
///
/// impl DepthFirstNode for Node {
///     fn children(&self) -> Vec<Self> {
///         [9, 7, 5, 3, 1].iter().map(|d| Node { value: 10 * self.value + d }).collect()
///     }
///
///     fn should_prune(&self) -> bool {
///         self.value >= 100
///     }
///
///     fn accept(&self) -> bool {
///         self.value > 0 && self.value % 13 == 0
///     }
/// }
///
/// let root = Node { value: 0 };
/// let numbers: Vec<u32> = DepthFirstSearcher::new(root).map(|node| node.value).collect();
///
/// assert_eq!(numbers, vec![117, 13, 195, 351, 377, 39, 533, 559, 715, 793, 91, 975]);
/// ```
pub struct DepthFirstSearcher<T: DepthFirstNode> {
    nodes_to_visit: Vec<T>,
}

impl<T: DepthFirstNode> DepthFirstSearcher<T> {
    /// Construct a new `DepthFirstSearcher` with the given root node.
    pub fn new(root: T) -> DepthFirstSearcher<T> {
        DepthFirstSearcher { nodes_to_visit: vec![root] }
    }
}

impl<T: DepthFirstNode> Iterator for DepthFirstSearcher<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some(node) = self.nodes_to_visit.pop() {
            if !node.should_prune() {
                self.nodes_to_visit.append(&mut node.children());
            }
            if node.accept() {
                return Some(node);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let constant = |_| 1;
        let identity = |n| n;
        let step = |n, x| if n < x { 0 } else { 1 };

        assert_eq!(binary_search(&constant, 0), 0);
        assert_eq!(binary_search(&constant, 1), 0);

        for x in 0..10 {
            assert_eq!(binary_search(&identity, x), x);
            assert_eq!(binary_search(&|n| step(n, x), 1), x);
        }
    }
}
