//! An iterator over the partition numbers

use numeric_traits::Algebraic;

/// A structure capable of iterating over the partition numbers.
///
/// The iteration is performed using Euler's pentagonal number theorem.
pub struct PartitionNumbersIterator<T: Algebraic + Copy> {
    previous_values: Vec<T>,
}

impl<T: Algebraic + Copy> Iterator for PartitionNumbersIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let n = self.previous_values.len();

        // The results for n = 0, 1 are the base cases for the recurrence.
        if n < 2 {
            self.previous_values.push(T::one());
        } else {
            let mut result = T::zero();

            // Add on the terms from positive-indexed pentagonal numbers.
            for (ix, pentagon) in (1..).map(|n| n * (3 * n - 1) / 2).take_while(|&x| x <= n).enumerate() {
                if ix % 2 == 0 {
                    result = result + self.previous_values[n - pentagon];
                } else {
                    result = result - self.previous_values[n - pentagon];
                }
            }

            // Add on the terms from negative-indexed pentagonal numbers.
            for (ix, pentagon) in (1..).map(|n| n * (3 * n + 1) / 2).take_while(|&x| x <= n).enumerate() {
                if ix % 2 == 0 {
                    result = result + self.previous_values[n - pentagon];
                } else {
                    result = result - self.previous_values[n - pentagon];
                }
            }

            self.previous_values.push(result);
        }

        self.previous_values.last().map(|x| *x)
    }
}

/// An iterator over the partition numbers.
pub fn partition_numbers<T: Algebraic + Copy>() -> PartitionNumbersIterator<T> {
    PartitionNumbersIterator {
        previous_values: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_numbers() {
        let partition_numbers = partition_numbers();
        assert_eq!(partition_numbers.take(14).collect::<Vec<u64>>(),
                   vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101]);
    }
}