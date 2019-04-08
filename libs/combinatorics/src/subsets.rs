//! Functions relating to subsets of an input set.

/// Execute the given function on each subset of the input data.
///
/// # Examples
///
/// ```
/// use combinatorics::each_subset;
///
/// let mut subsets: Vec<Vec<usize>> = vec![];
/// let mut data = vec![1, 2, 3];
///
/// each_subset(&data, |subset| subsets.push(subset.to_vec()));
///
/// assert_eq!(subsets, vec![
///     vec![],
///     vec![3],
///     vec![2],
///     vec![2, 3],
///     vec![1],
///     vec![1, 3],
///     vec![1, 2],
///     vec![1, 2, 3],
/// ]);
/// ```
pub fn each_subset<T: Clone, F: FnMut(&[T])>(data: &[T], mut f: F) {
    _each_subset_inner(data, &mut f, &mut Vec::new(), 0);
}

fn _each_subset_inner<T: Clone, F: FnMut(&[T])>(data: &[T], f: &mut F, elements: &mut Vec<T>, k: usize) {
    if k == data.len() { f(elements); }
    else {
        _each_subset_inner(data, f, elements, k + 1);
        elements.push(data[k].clone());
        _each_subset_inner(data, f, elements, k + 1);
        elements.pop();
    }
}
