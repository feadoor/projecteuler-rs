//! A modulo 30 wheel - iterates over the differences between successive multiples of a number
//! which are non-multiples of 2, 3 and 5.

const NUM_DIFFS: usize = 8;

/// Keeps track of the current state of the wheel.
pub struct Wheel30 {
    /// The index of the current difference.
    curr_ix: usize,
    /// The differences that should be cyclically yielded.
    diffs: [u64; NUM_DIFFS],
}

impl Wheel30 {
    /// Creates a wheel from the given base number and starting multiple.
    ///
    /// For example, with num = 7 and mult = 11, the starting multiple is 77, and the first few
    /// differences are 14, 28, 14, 28, 42...
    pub fn new(num: u64, mult: u64) -> Wheel30 {
        let ix = match mult % 30 {
            1 => 7,
            7 => 0,
            11 => 1,
            13 => 2,
            17 => 3,
            19 => 4,
            23 => 5,
            29 => 6,
            _ => unreachable!(),
        };
        let diffs = [6 * num, 4 * num, 2 * num, 4 * num, 2 * num, 4 * num, 6 * num, 2 * num];
        Wheel30 {
            curr_ix: ix,
            diffs: diffs,
        }
    }

    /// Returns the next difference from the wheel.
    #[inline]
    pub fn next_diff(&mut self) -> u64 {
        self.curr_ix += 1;
        if self.curr_ix == NUM_DIFFS {
            self.curr_ix = 0;
        }
        self.diffs[self.curr_ix]
    }
}

#[cfg(test)]
mod tests {
    use super::Wheel30;

    #[test]
    fn test_wheel() {
        let mut wheel = Wheel30::new(7, 11);
        assert_eq!(wheel.next_diff(), 14);
        assert_eq!(wheel.next_diff(), 28);
        assert_eq!(wheel.next_diff(), 14);
        assert_eq!(wheel.next_diff(), 28);
        assert_eq!(wheel.next_diff(), 42);
        assert_eq!(wheel.next_diff(), 14);
        assert_eq!(wheel.next_diff(), 42);
        assert_eq!(wheel.next_diff(), 28);
        assert_eq!(wheel.next_diff(), 14);
    }
}