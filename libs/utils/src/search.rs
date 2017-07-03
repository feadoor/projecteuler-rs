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
