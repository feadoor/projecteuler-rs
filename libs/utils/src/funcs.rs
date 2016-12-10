//! Implementations of useful utility functions.

/// Finds the smallest value of `n` for which func(n) is at least the target value using a binary
/// search. Assumes that such a value exists, and that the function is increasing.
///
/// # Examples
///
/// ```
/// use utils::binary_search;
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

/// Rearranges the elements of the given slice to the next permutation in lexicographic order.
///
/// Shuffles the slice in-place. Returns `true` if a rearrangement took place, and `false` if the
/// current ordering was maximal according to lexicographic ordering.
///
/// Use a classical algorithm that can be found in Knuth's 'The Art of Computer Programming'. It
/// runs as follows:
///
/// <ol>
/// <li>Identify the smallest j such that the a[j+1] onwards are in reverse order.</li>
/// <li>Swap a[j] with the smallest element right of a[j] which is larger than it</li>
/// <li>Reverse everything to the right of a[j]</li>
/// </ol>
///
/// # Examples
///
/// ```
/// use utils::permute_next;
///
/// let mut arr = vec![1, 1, 2, 3];
///
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![1, 1, 3, 2]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![1, 2, 1, 3]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![1, 2, 3, 1]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![1, 3, 1, 2]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![1, 3, 2, 1]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![2, 1, 1, 3]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![2, 1, 3, 1]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![2, 3, 1, 1]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![3, 1, 1, 2]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![3, 1, 2, 1]);
/// assert_eq!(permute_next(&mut arr), true); assert_eq!(arr, vec![3, 2, 1, 1]);
/// assert_eq!(permute_next(&mut arr), false); assert_eq!(arr, vec![3, 2, 1, 1]);
/// ```
pub fn permute_next<T: Ord>(arr: &mut [T]) -> bool {

    // Return false for an empty slice
    if arr.len() == 0 { return false; }

    // Identify the index j.
    let j_plus_one = (0..arr.len() - 1).rev().take_while(|&x| arr[x] >= arr[x + 1]).last();
    let j = match j_plus_one {
        None => if arr.len() > 1 { arr.len() - 2 } else { return false },
        Some(0) => return false,
        Some(x) => x - 1,
    };

    // Find the index k and swap a[j] with a[k]
    let k_plus_one = (j + 1..arr.len()).rev().take_while(|&x| arr[j] >= arr[x]).last();
    let k = match k_plus_one {
        None => arr.len() - 1,
        Some(x) => x - 1,
    };
    arr.swap(j, k);

    // Reverse everything past arr[j] in the resulting permutation.
    arr[j+1..].reverse();
    true
}

/// Rearranges the elements of the given slice to the previous permutation in lexicographic order.
///
/// Shuffles the slice in-place. Returns `true` if a rearrangement took place, and `false` if the
/// current ordering was maximal according to lexicographic ordering.
///
/// Use a classical algorithm that can be found in Knuth's 'The Art of Computer Programming'. It
/// runs as follows:
///
/// <ol>
/// <li>Identify the smallest j such that the a[j+1] onwards are in order.</li>
/// <li>Swap a[j] with the largest element right of a[j] which is smaller than it</li>
/// <li>Reverse everything to the right of a[j]</li>
/// </ol>
///
/// # Examples
///
/// ```
/// use utils::permute_prev;
///
/// let mut arr = vec![3, 2, 1, 1];
///
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![3, 1, 2, 1]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![3, 1, 1, 2]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![2, 3, 1, 1]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![2, 1, 3, 1]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![2, 1, 1, 3]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 3, 2, 1]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 3, 1, 2]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 2, 3, 1]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 2, 1, 3]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 1, 3, 2]);
/// assert_eq!(permute_prev(&mut arr), true); assert_eq!(arr, vec![1, 1, 2, 3]);
/// assert_eq!(permute_prev(&mut arr), false); assert_eq!(arr, vec![1, 1, 2, 3]);
/// ```
pub fn permute_prev<T: Ord>(arr: &mut [T]) -> bool {

    // Return false for an empty slice
    if arr.len() == 0 { return false; }

    // Identify the index j.
    let j_plus_one = (0..arr.len() - 1).rev().take_while(|&x| arr[x] <= arr[x + 1]).last();
    let j = match j_plus_one {
        None => if arr.len() > 1 { arr.len() - 2 } else { return false },
        Some(0) => return false,
        Some(x) => x - 1,
    };

    // Find the index k and swap a[j] with a[k]
    let k_plus_one = (j + 1..arr.len()).rev().take_while(|&x| arr[j] <= arr[x]).last();
    let k = match k_plus_one {
        None => arr.len() - 1,
        Some(x) => x - 1,
    };
    arr.swap(j, k);

    // Reverse everything past arr[j] in the resulting permutation.
    arr[j+1..].reverse();
    true
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

    #[test]
    fn test_permute_next() {
        let permutations = |mut arr: &mut [u64]| {
            let mut perms = Vec::new();
            loop {
                perms.push(arr.to_vec());
                if !permute_next(&mut arr) {
                    break;
                }
            }
            perms
        };

        assert_eq!(permutations(&mut []), vec![[]]);

        assert_eq!(permutations(&mut [1]), vec![[1]]);

        assert_eq!(permutations(&mut [1, 2, 3]),
                   vec![[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);

        assert_eq!(permutations(&mut [1, 3, 2, 1]),
                   vec![[1, 3, 2, 1], [2, 1, 1, 3], [2, 1, 3, 1], [2, 3, 1, 1], [3, 1, 1, 2],
                        [3, 1, 2, 1], [3, 2, 1, 1]]);
    }

    #[test]
    fn test_permute_prev() {
        let permutations = |mut arr: &mut [u64]| {
            let mut perms = Vec::new();
            loop {
                perms.push(arr.to_vec());
                if !permute_prev(&mut arr) {
                    break;
                }
            }
            perms
        };

        assert_eq!(permutations(&mut []), vec![[]]);

        assert_eq!(permutations(&mut [1]), vec![[1]]);

        assert_eq!(permutations(&mut [3, 2, 1]),
                   vec![[3, 2, 1], [3, 1, 2], [2, 3, 1], [2, 1, 3], [1, 3, 2], [1, 2, 3]]);

        assert_eq!(permutations(&mut [2, 1, 1, 3]),
                   vec![[2, 1, 1, 3], [1, 3, 2, 1], [1, 3, 1, 2], [1, 2, 3, 1], [1, 2, 1, 3],
                        [1, 1, 3, 2], [1, 1, 2, 3]]);
    }
}
