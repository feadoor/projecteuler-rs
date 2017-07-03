//! Implementations of useful utility functions.

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

/// A structure which is used for iterating through a tree, depth-first, producing only those nodes
/// which satisfy a particular condition.
pub struct DepthFirstSearcher<'a, T: 'a> {
    nodes_to_visit: Vec<T>,
    children: &'a Fn(&T) -> Vec<T>,
    should_prune: &'a Fn(&T) -> bool,
    accept: &'a Fn(&T) -> bool,
}

impl<'a, T> Iterator for DepthFirstSearcher<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some(node) = self.nodes_to_visit.pop() {
            if !(self.should_prune)(&node) {
                self.nodes_to_visit.append(&mut (self.children)(&node));
                if (self.accept)(&node) {
                    return Some(node);
                }
            }
        }

        None
    }
}

/// Return a structure for iterating over all nodes in a depth-first structure which satisfy a
/// particular condition.
///
/// Takes an item which is taken to be the root of the tree, a function which produces the children
/// of a given node, a function `should_prune` which returns `true` if the tree can be pruned above
/// the given node, and a function `accept` which returns `true` if the current node should be
/// accepted.
///
/// # Examples
///
/// ```
/// use utils::search::depth_first_search;
///
/// // Find all numbers below 666 with only odd digits and divisible by 13.
/// let root = 0;
/// let children = |x: &u32| [9, 7, 5, 3, 1].iter().map(|d| 10 * x + d).collect();
/// let should_prune = |x: &u32| *x >= 666;
/// let accept = |x: &u32| *x > 0 && *x % 13 == 0;
///
/// let numbers: Vec<u32> = depth_first_search(root, &children, &should_prune, &accept).collect();
/// assert_eq!(numbers, vec![117, 13, 195, 351, 377, 39, 533, 559, 91]);
/// ```
pub fn depth_first_search<'a, T> (
    root: T,
    children: &'a Fn(&T) -> Vec<T>,
    should_prune: &'a Fn(&T) -> bool,
    accept: &'a Fn(&T) -> bool
) -> DepthFirstSearcher<'a, T> {
    DepthFirstSearcher {
        nodes_to_visit: vec![root],
        children: children,
        should_prune: should_prune,
        accept: accept,
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
