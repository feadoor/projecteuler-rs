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

/// A trait for a tree which is to be traversed, depth-first, in the pursuit of nodes which
/// satisfy a particular condition.
///
/// # Examples
///
/// ```
/// use utils::search::DepthFirstTree;
///
/// // Find all numbers below 666 with only odd digits and divisible by 13.
/// struct Node {
///     value: u32,
/// }
///
/// struct Tree {
///     max_value: u32,
/// }
///
/// impl DepthFirstTree for Tree {
///     type Node = Node;
///
///     fn roots(&self) -> Vec<Node> {
///         vec![Node { value: 0 }]
///     }
///
///     fn children(&mut self, node: &Node) -> Vec<Node> {
///         [1, 3, 5, 7, 9].iter().map(|d| Node { value: 10 * node.value + d }).collect()
///     }
///
///     fn should_prune(&mut self, node: &Node) -> bool {
///         node.value >= 100
///     }
///
///     fn accept(&mut self, node: &Node) -> bool {
///         node.value > 0 && node.value < self.max_value && node.value % 13 == 0
///     }
/// }
///
/// let mut tree = Tree { max_value: 666 };
/// let numbers: Vec<_> = tree.iter().map(|node| node.value).collect();
///
/// assert_eq!(numbers, vec![117, 13, 195, 351, 377, 39, 533, 559, 91]);
/// ```
pub trait DepthFirstTree<> where Self: Sized {
    type Node: Sized;

    /// Returns the roots of the tree.
    fn roots(&self) -> Vec<Self::Node>;
    /// Returns all the nodes which are direct descendants of this node.
    fn children(&mut self, node: &Self::Node) -> Vec<Self::Node>;
    /// Update any tree-wide state before examining the given node.
    #[allow(unused_variables)]
    fn update_state_before(&mut self, node: &Self::Node) {}
    // Update any tree-wide state after having examined the given node, and all of its children.
    #[allow(unused_variables)]
    fn update_state_after(&mut self, node: &Self::Node) {}
    /// Returns `true` if the search tree can be pruned below this node - that is, none of its
    /// children can possibly satisfy the condition.
    fn should_prune(&mut self, node: &Self::Node) -> bool;
    /// Returns `true` if this node satisfies the condition.
    fn accept(&mut self, node: &Self::Node) -> bool;

    /// An iterator over the nodes of the tree which meet the condition.
    fn iter(&mut self) -> DepthFirstSearcher<Self> {
        let roots = self.roots().into_iter().map(|node| DFSNode::Down(node)).collect();
        DepthFirstSearcher { tree: self, nodes_to_check: roots }
    }
}


/// An enum for distinguishing between the two times we see a node in DFS - first, when going
/// "down" through the node initially, and secondly, when going "up" through the node when
/// backtracking.
enum DFSNode<T: DepthFirstTree> {
    Down(T::Node),
    Up(T::Node, bool),
}

/// A structure which is used for iterating through a tree, depth-first, producing only those nodes
/// which satisfy a particular condition.
pub struct DepthFirstSearcher<'a, T: 'a + DepthFirstTree> {
    nodes_to_check: Vec<DFSNode<T>>,
    tree: &'a mut T,
}

impl<'a, T: DepthFirstTree> Iterator for DepthFirstSearcher<'a, T> {
    type Item = T::Node;

    fn next(&mut self) -> Option<T::Node> {
        use self::DFSNode::{Down, Up};

        while let Some(node) = self.nodes_to_check.pop() {
            match node {
                Down(node) => {
                    // Update global tree state.
                    self.tree.update_state_before(&node);

                    // Prepare child nodes and check if this node is acceptable.
                    let children = if self.tree.should_prune(&node) { Vec::new() } else { self.tree.children(&node) };
                    let accept = self.tree.accept(&node);

                    // Update the tree and the list of nodes to be visited.
                    self.nodes_to_check.push(Up(node, accept));
                    self.nodes_to_check.extend(children.into_iter().rev().map(|child| Down(child)));
                },
                Up(node, accept) => {
                    self.tree.update_state_after(&node);
                    if accept { return Some(node); }
                }
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
