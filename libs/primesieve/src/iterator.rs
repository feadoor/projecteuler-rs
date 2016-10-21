//! Iteration over the numbers encoded in a sieve.

const MODULUS: u64 = 240;
const OFFSETS: &'static [u64; 64] =
    &[1, 7, 11, 13, 17, 19, 23, 29,
      31, 37, 41, 43, 47, 49, 53, 59,
      61, 67, 71, 73, 77, 79, 83, 89,
      91, 97, 101, 103, 107, 109, 113, 119,
      121, 127, 131, 133, 137, 139, 143, 149,
      151, 157, 161, 163, 167, 169, 173, 179,
      181, 187, 191, 193, 197, 199, 203, 209,
      211, 217, 221, 223, 227, 229, 233, 239];

/// A structure which iterates over the numbers represented by a given sequence of integers using
/// the encoding described in the module `segment`.
pub struct SieveIterator<'a> {
    /// The current `u64` we are extracting numbers from.
    current: u64,
    /// The offset to add to the numbers we extract from the current `u64`.
    base: u64,
    /// The index in the sieve of the current `u64`.
    curr_idx: usize,
    /// The sieve encoding the numbers to iterate over.
    sieve: &'a [u64],
}

impl<'a> SieveIterator<'a> {
    /// Create a new `SieveIterator` which is ready to iterate over the numbers encoded in the
    /// given sieve of `u64`s.
    pub fn new(sieve: &'a [u64]) -> SieveIterator {
        if sieve.len() == 0 {
            SieveIterator {
                current: 0,
                base: 0,
                curr_idx: 0,
                sieve: sieve,
            }
        } else {
            SieveIterator {
                current: sieve[0],
                base: 0,
                curr_idx: 0,
                sieve: sieve,
            }
        }
    }
}

impl<'a> Iterator for SieveIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // If all the numbers from the current `u64` have been considered, look for the next `u64`
        // which encodes a number.
        if self.current == 0 {
            for idx in self.curr_idx + 1..self.sieve.len() {
                self.base += MODULUS;
                if self.sieve[idx] != 0 {
                    self.current = self.sieve[idx];
                    self.curr_idx = idx;
                    break;
                }
            }
        }

        // If we've exhausted all the numbers, indicate so.
        if self.current == 0 {
            return None;
        }

        // Get the next number from the current `u64`.
        let bit = self.current.trailing_zeros();
        self.current &= self.current - 1;
        Some(self.base + OFFSETS[bit as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(SieveIterator::new(&[]).collect::<Vec<u64>>(), vec![]);
    }

    #[test]
    fn test_small() {
        let sieve = [0b1001001100101100000001011010010];
        let iter = SieveIterator::new(&sieve);
        assert_eq!(iter.collect::<Vec<u64>>(),
                   vec![7, 17, 23, 29, 37, 67, 71, 77, 89, 91, 103, 113]);
    }

    #[test]
    fn test_medium() {
        let sieve = [0b1001001100101100000001011010010, 0b0, 0b1100101100000001011010010];
        let iter = SieveIterator::new(&sieve);
        assert_eq!(iter.collect::<Vec<u64>>(),
                   vec![7, 17, 23, 29, 37, 67, 71, 77, 89, 91, 103, 113,
                        487, 497, 503, 509, 517, 547, 551, 557, 569, 571]);
    }
}