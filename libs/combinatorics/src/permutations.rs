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
        else if !next_permutation(&mut self.items) { return None; }
        Some(self.items.clone())
    }
}

/// Execute the given function on each permutation of the input data.
///
/// # Examples
///
/// ```
/// use combinatorics::each_permutation;
///
/// let mut perms: Vec<Vec<usize>> = vec![];
/// let mut data = vec![1, 2, 3];
///
/// each_permutation(&mut data, |perm| perms.push(perm.to_vec()));
///
/// assert_eq!(perms, vec![
///     vec![1, 2, 3],
///     vec![1, 3, 2],
///     vec![2, 1, 3],
///     vec![2, 3, 1],
///     vec![3, 1, 2],
///     vec![3, 2, 1],
/// ]);
/// ```
pub fn each_permutation<T: Ord, F: FnMut(&[T])>(data: &mut [T], mut f: F) {
    data.sort(); f(data);
    while next_permutation(data) {
        f(data);
    }
}

/// Advance the input slice to the next lexicographical permutation.
fn next_permutation<T: Ord>(data: &mut [T]) -> bool {

    // Find the smallest `j` such that all permutations beginning with `a₁, ... , aⱼ` have
    // already been visited.
    let mut j = data.len() - 2;
    while j > 0 && data[j] >= data[j + 1] { j -= 1; }
    if data[j] >= data[j + 1] { return false; }

    // Find the index `k` so that `aₖ` goes immediately to the right of `aⱼ` and swap them
    let mut k = data.len() - 1;
    while data[j] >= data[k] { k -= 1; }
    data.swap(j, k);

    // Reverse the suffix of the resulting permutation
    data[j + 1..].reverse();
    true
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
