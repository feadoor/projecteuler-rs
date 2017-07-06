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
/// # Concepts
///
/// There are two concepts that are important in understanding how to use this trait - that of
/// tree "state", and that of tree "steps".
///
/// The "state" of a tree is an umbrella term encompassing all the information held about a
/// particular position in the tree - including any values associated with the position and any
/// internal variables that are used to aid the search.
///
/// A tree "step" is a small structure - ideally one that can be constructed very cheaply - which
/// contains instructions about how to move between parent/child tree states.
///
/// # Usage
///
/// To define a tree, you must provide the following:
///
///  - A structure holding the tree state, which you will implement `DepthFirstTree` for
///  - An associated type `Step` which represents the tree "steps"
///  - An associated type `Output` which represents the outputs of the search
///  - Implementations of the required methods of this trait:
///      - `next_steps` must return the steps that it is valid to take from the current state
///      - `should_prune` must decide whether to prune the tree at the current state
///      - `apply_step` must define how to use a step to go from parent to child state
///      - `revert_step` must define how to use a step to go from child back to parent state
///      - `output` must take the current state of the tree, and return `Some(value)` if the current
///         state meets the condition.
///
/// # Examples
///
/// In this example, we will search for all numbers having at most 3 digits, with all digits being
/// odd, and the whole number being divisible by 13.
///
/// We will utilise a tree search, with child states coming from the parent by appending a single
/// digit to the parent state.
///
/// Our tree state will be the current number being examined, the current number of digits,
/// and the maximum number of digits that a solution should have (in this case, 3).
///
/// Our steps will simply contain the digit to be appended when going from parent to child.
///
/// ```
/// use utils::search::{DepthFirstTree, Pruning};
///
/// struct Step {
///     next_digit: u32,
/// }
///
/// struct Tree {
///     value: u32,
///     number_of_digits: usize,
///     max_digits: usize,
/// }
///
/// impl Tree {
///     fn new(max_digits: usize) -> Tree {
///         Tree { value: 0, number_of_digits: 0, max_digits: max_digits }
///     }
/// }
///
/// impl DepthFirstTree for Tree {
///     type Step = Step;
///     type Output = u32;
///
///     /// All possible choices for the next digit, which must be odd, as per our constraints.
///     fn next_steps(&mut self) -> Vec<Step> {
///         [1, 3, 5, 7, 9].iter().map(|&digit| { Step { next_digit: digit } }).collect()
///     }
///
///     /// Prune the tree if we have reached the maximum number of digits.
///     fn should_prune(&mut self) -> Pruning {
///         if self.number_of_digits == self.max_digits {
///             Pruning::Below
///         } else {
///             Pruning::None
///         }
///     }
///
///     /// Append the next digit to the end of the current value.
///     fn apply_step(&mut self, step: &Step) {
///         self.value = 10 * self.value + step.next_digit;
///         self.number_of_digits += 1;
///     }
///
///     /// Remove the last digit from the end of the current value.
///     fn revert_step(&mut self, step: &Step) {
///         self.value /= 10;
///         self.number_of_digits -= 1;
///     }
///
///     /// Check if the value is divisible by 13,and output it if so.
///     fn output(&mut self) -> Option<u32> {
///         if self.value > 0 && self.value % 13 == 0 {
///             Some(self.value)
///         } else {
///             None
///         }
///     }
/// }
///
/// let numbers: Vec<_> = Tree::new(3).iter().collect();
/// assert_eq!(numbers, vec![117, 13, 195, 351, 377, 39, 533, 559, 715, 793, 91, 975]);
/// ```
pub trait DepthFirstTree where Self: Sized {
    type Step: Sized;
    type Output;

    /// Returns the steps that can be taken from the current state of the tree.
    fn next_steps(&mut self) -> Vec<Self::Step>;
    /// Returns whether to prune the tree at the current state.
    fn should_prune(&mut self) -> Pruning;
    /// Update the tree-wide state as a result of applying the given step.
    fn apply_step(&mut self, node: &Self::Step);
    /// Update any tree-wide state by reverting the given step.
    fn revert_step(&mut self, node: &Self::Step);
    /// Maps the internal state of the tree to the actual form required for output.
    ///
    /// Returns `Some(value)` if the current state satisfies the condition, and `None` otherwise.
    fn output(&mut self) -> Option<Self::Output>;

    /// An iterator over the nodes of the tree which meet the condition.
    fn iter(&mut self) -> DepthFirstSearcher<Self> {
        DepthFirstSearcher::new(self)
    }
}

/// An enum for the different types of pruning that can be applied at a given point in the tree.
pub enum Pruning {
    /// Prune the tree above the current state - discount this state, and all of its descendants.
    Above,
    /// Prune the tree below the current state - discount all descendants, but keep this state.
    Below,
    /// Do not prune the tree at this state.
    None,
}


/// An enum for distinguishing between the two times we see a step in DFS - first, when going
/// "down" through the step initially, and secondly, when going "up" through the step when
/// backtracking.
enum Step<T: DepthFirstTree> {
    Apply(T::Step),
    Revert(T::Step),
    StartSearch,
    EndSearch,
}

/// A structure which is used for iterating through a tree, depth-first, producing only those nodes
/// which satisfy a particular condition.
pub struct DepthFirstSearcher<'a, T: 'a + DepthFirstTree> {
    tree: &'a mut T,
    steps: Vec<Step<T>>,
}

impl<'a, T: 'a + DepthFirstTree> DepthFirstSearcher<'a, T> {
    /// Construct a new `DepthFirstSearcher` which will examine the tree below the current state.
    fn new(tree: &'a mut T) -> DepthFirstSearcher<'a, T> {
        DepthFirstSearcher { tree: tree, steps: vec![Step::StartSearch] }
    }

    /// Apply the given step, and record how to revert it after all its descendants are searched.
    fn apply_step(&mut self, step: T::Step) {
        self.tree.apply_step(&step);
        self.steps.push(Step::Revert(step));
    }

    /// Add all steps required to reach the children of the current state.
    fn add_child_steps(&mut self) {
        self.steps.extend(self.tree.next_steps().into_iter().rev().map(|step| Step::Apply(step)));
    }
}

impl<'a, T: DepthFirstTree> Iterator for DepthFirstSearcher<'a, T> {
    type Item = T::Output;

    fn next(&mut self) -> Option<T::Output> {
        use self::Step::*;

        while let Some(step) = self.steps.pop() {
            match step {
                StartSearch => {
                    self.steps.push(EndSearch);
                    self.add_child_steps();
                },
                EndSearch => {
                    if let Some(value) = self.tree.output() {
                        return Some(value);
                    }
                },
                Apply(step) => {
                    self.apply_step(step);
                    match self.tree.should_prune() {
                        Pruning::Above => {
                            if let Some(Revert(step)) = self.steps.pop() {
                                self.tree.revert_step(&step);
                            }
                        },
                        Pruning::Below => {},
                        Pruning::None => { self.add_child_steps(); },
                    }
                },
                Revert(step) => {
                    let output = self.tree.output();
                    self.tree.revert_step(&step);
                    if let Some(value) = output {
                        return Some(value);
                    }
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
