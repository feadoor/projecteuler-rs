//! Extra iterator adaptors and utility methods

mod combinations_with_replacement;
mod lazy_buffer;
mod permutations;

pub trait CombinationsWithReplacement : Iterator {

    /// Return an iterator adaptor that iterates over the size-`n` combinations of elements from the
    /// input iterator, allowing repeated elements within each combination.
    ///
    /// The iterator item type is `Vec<Self::Item>`, and a new `Vec` is produced on each iteration,
    /// cloning the original iterator elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use iterators::CombinationsWithReplacement;
    ///
    /// let mut it = (1..5).combinations_with_replacement(3);
    ///
    /// assert_eq!(it.next(), Some(vec![1, 1, 1]));
    /// assert_eq!(it.next(), Some(vec![1, 1, 2]));
    /// assert_eq!(it.next(), Some(vec![1, 1, 3]));
    /// assert_eq!(it.next(), Some(vec![1, 1, 4]));
    /// assert_eq!(it.next(), Some(vec![1, 2, 2]));
    /// assert_eq!(it.next(), Some(vec![1, 2, 3]));
    /// assert_eq!(it.next(), Some(vec![1, 2, 4]));
    /// assert_eq!(it.next(), Some(vec![1, 3, 3]));
    /// assert_eq!(it.next(), Some(vec![1, 3, 4]));
    /// assert_eq!(it.next(), Some(vec![1, 4, 4]));
    /// assert_eq!(it.next(), Some(vec![2, 2, 2]));
    /// assert_eq!(it.next(), Some(vec![2, 2, 3]));
    /// assert_eq!(it.next(), Some(vec![2, 2, 4]));
    /// assert_eq!(it.next(), Some(vec![2, 3, 3]));
    /// assert_eq!(it.next(), Some(vec![2, 3, 4]));
    /// assert_eq!(it.next(), Some(vec![2, 4, 4]));
    /// assert_eq!(it.next(), Some(vec![3, 3, 3]));
    /// assert_eq!(it.next(), Some(vec![3, 3, 4]));
    /// assert_eq!(it.next(), Some(vec![3, 4, 4]));
    /// assert_eq!(it.next(), Some(vec![4, 4, 4]));
    /// assert_eq!(it.next(), None);
    /// ```
    fn combinations_with_replacement(self, n: usize) -> combinations_with_replacement::CombinationsWithReplacement<Self>
        where Self: Sized, Self::Item: Clone {
        combinations_with_replacement::combinations_with_replacement(self, n)
    }
}

pub trait Permutations : Iterator {

    /// Return an iterator adaptor that iterates over permutations of elements from the input
    /// taking into account that some elements may be equal.
    ///
    /// The iterator item type is `Vec<Self::Item>`, and a new `Vec` is produced on each iteration,
    /// cloning the original iterator elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use iterators::Permutations;
    ///
    /// let numbers = vec![1, 1, 2, 3];
    /// let mut it = numbers.iter().map(|x| *x).permutations();
    ///
    /// assert_eq!(it.next(), Some(vec![1, 1, 2, 3]));
    /// assert_eq!(it.next(), Some(vec![1, 1, 3, 2]));
    /// assert_eq!(it.next(), Some(vec![1, 2, 1, 3]));
    /// assert_eq!(it.next(), Some(vec![1, 2, 3, 1]));
    /// assert_eq!(it.next(), Some(vec![1, 3, 1, 2]));
    /// assert_eq!(it.next(), Some(vec![1, 3, 2, 1]));
    /// assert_eq!(it.next(), Some(vec![2, 1, 1, 3]));
    /// assert_eq!(it.next(), Some(vec![2, 1, 3, 1]));
    /// assert_eq!(it.next(), Some(vec![2, 3, 1, 1]));
    /// assert_eq!(it.next(), Some(vec![3, 1, 1, 2]));
    /// assert_eq!(it.next(), Some(vec![3, 1, 2, 1]));
    /// assert_eq!(it.next(), Some(vec![3, 2, 1, 1]));
    /// assert_eq!(it.next(), None);
    /// ```
    fn permutations(self) -> permutations::Permutations<Self>
        where Self: Sized, Self::Item: Clone + Ord {
        permutations::permutations(self)
    }
}

impl<I: Iterator> CombinationsWithReplacement for I where I::Item: Clone { }
impl<I: Iterator> Permutations for I where I::Item: Clone + Ord { }
