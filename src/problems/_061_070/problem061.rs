//! [Problem 61 (Cyclical figurate numbers)](https://projecteuler.net/problem=61)
//!
//! # Solution detail
//!
//! We can use a depth-first search, starting with octagonal numbers as the first-level nodes,
//! and having child nodes formed from parent nodes by adding one more polygonal number (meeting
//! the digit constraints) to the set. It is possible to assume that we should start with octagonal
//! numbers because solutions are, by definition, cyclic, so any solution can be rotated to one that
//! starts with an octagonal number.
//!
//! The search can be sped up by precomputing lists of four-digit polygonal numbers, so that they
//! do not need to be recalculated every time we take a step.

#[macro_use]
extern crate projecteuler_rs;
extern crate search;

use search::{DepthFirstTree, Pruning};

fn digits_match(prev: u64, next: u64) -> bool {
    prev % 100 == next / 100
}

/// A structure that can be used for conducting a depth-first search for sets of cyclic polygonal
/// numbers.
struct CyclicPolygonalTree {
    /// The size of the set that we are looking for.
    required_size: usize,
    /// Precomputed lists of polygonal numbers having the right number of digits.
    polygonal_numbers: Vec<Vec<u64>>,
    /// A boolean array indicating which types of polygonal number have already been used.
    polygonal_types_used: Vec<bool>,
    /// The set of numbers currently being considered.
    current_set: Vec<u64>,
}

impl CyclicPolygonalTree {
    /// Constructs a new `CyclicPolygonalTree` which will search for cyclic sets of 4-digit
    /// polygonal numbers of the given size.
    fn with_required_size(size: usize) -> CyclicPolygonalTree {
        let (min_value, max_value) = (1_000, 10_000);

        let polygonal = |d: u64, n: u64| n * ((d - 2) * n + 4 - d) / 2;
        let polygonals_in_range = |d| (1..)
            .map(|n| polygonal(d, n))
            .filter(|&p| p >= min_value)
            .take_while(|&p| p < max_value)
            .collect();

        let polygonal_numbers = (3..size + 3)
            .map(|d| polygonals_in_range(d as u64))
            .collect();

        CyclicPolygonalTree {
            required_size: size,
            polygonal_numbers: polygonal_numbers,
            polygonal_types_used: vec![false; size],
            current_set: Vec::new(),
        }
    }
}

/// A description of a step that can be taken in the search tree.
struct CyclicPolygonalTreeStep {
    /// The next number to be added to the set.
    next_number: u64,
    /// The index of the type of polygonal numbers that this number came from.
    polygonal_type: usize,
}

/// Search for cyclic sets of polygonal numbers meeting the given constraints.
impl DepthFirstTree for CyclicPolygonalTree {
    type Step = CyclicPolygonalTreeStep;
    type Output = Vec<u64>;

    /// All possible choices of the next number to put into the set.
    fn next_steps(&mut self) -> Vec<Self::Step> {
        let mut steps = Vec::new();

        if self.current_set.len() == 0 {
            let idx = self.required_size - 1;
            steps = self.polygonal_numbers[idx].iter().map(|&n| (n, idx)).collect();
        } else {
            let last_number = *self.current_set.last().unwrap();
            for (idx, numbers) in self.polygonal_numbers.iter().enumerate() {
                if !self.polygonal_types_used[idx] {
                    steps.extend(numbers.iter()
                        .filter(|&&n| digits_match(last_number, n))
                        .map(|&n| (n, idx))
                    );
                }
            }
        }

        steps.iter()
            .map(|&(n, idx)| CyclicPolygonalTreeStep { next_number: n, polygonal_type: idx })
            .collect()
    }

    /// The pruning actually happens as a side-effect of generating the next steps.
    fn should_prune(&mut self) -> Pruning {
        Pruning::None
    }

    /// Add the next number to the end of the current set.
    fn apply_step(&mut self, step: &Self::Step) {
        self.current_set.push(step.next_number);
        self.polygonal_types_used[step.polygonal_type] = true;
    }

    /// Remove the last number from the current set.
    fn revert_step(&mut self, step: &Self::Step) {
        self.current_set.pop();
        self.polygonal_types_used[step.polygonal_type] = false;
    }

    /// Output the current set, if it is of the right length, and also satisfies the cyclic property
    /// on the final and initial numbers.
    fn output(&mut self) -> Option<Self::Output> {
        if self.current_set.len() == self.required_size &&
           digits_match(self.current_set[self.required_size - 1], self.current_set[0]) {
            Some(self.current_set.clone())
        } else {
            None
        }
    }
}


/// Find the sum of all cyclic sequences of 4-digit polygonal numbers having the given size.
fn solve(size_of_set: usize) -> u64 {
    let searcher = CyclicPolygonalTree::with_required_size(size_of_set);
    searcher.into_iter().map(|set| set.iter().sum::<u64>()).sum()
}

/// Solve the problem, returning the answer as a `String`
fn answer() -> String {
    solve(6).to_string()
}

problem!(answer, "28684");
