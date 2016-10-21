//! Methods for dealing with segments of a segmented sieve represented in a memory-efficient way.
//!
//! # Overview
//!
//! A segment represents the numbers in a given range which are prime. The range must begin and
//! end on a multiple of 240, due to the way that the segment is represented internally, and is
//! indexed from 0, so that the zeroth element is the beginning of the range, and so on. In
//! reality, the only indices which make sense are those which are 1, 7, 11, 13, 17, 19, 23 or 29
//! more than a multiple of 30, and any other indices will always return `false`.
//!
//! # Details
//!
//! Each group of 30 numbers is represented as a single byte. This is possible since, by
//! eliminating multiples of 2, 3 and 5, only 8 numbers in the given segment can possibly be
//! primes - we need not bother to store any information for the other numbers. These bytes are
//! grouped together and stored internally as 64-bit integers.

pub const MODULUS: u64 = 240;

/// Calculate the internal index at which the bit for a given index into the range is found.
#[inline]
fn index_for(idx: u64) -> (bool, usize, u64) {
    const POS: &'static [(bool, u64); MODULUS as usize] =
          // 0
        &[(false, 1 << 0), (true, 1 << 0), (false, 1 << 1), (false, 1 << 1), (false, 1 << 1),
          (false, 1 << 1), (false, 1 << 1), (true, 1 << 1), (false, 1 << 2), (false, 1 << 2),
          (false, 1 << 2), (true, 1 << 2), (false, 1 << 3), (true, 1 << 3), (false, 1 << 4),
          (false, 1 << 4), (false, 1 << 4), (true, 1 << 4), (false, 1 << 5), (true, 1 << 5),
          (false, 1 << 6), (false, 1 << 6), (false, 1 << 6), (true, 1 << 6), (false, 1 << 7),
          (false, 1 << 7), (false, 1 << 7), (false, 1 << 7), (false, 1 << 7), (true, 1 << 7),
          // 30
          (false, 1 << 8), (true, 1 << 8), (false, 1 << 9), (false, 1 << 9), (false, 1 << 9),
          (false, 1 << 9), (false, 1 << 9), (true, 1 << 9), (false, 1 << 10), (false, 1 << 10),
          (false, 1 << 10), (true, 1 << 10), (false, 1 << 11), (true, 1 << 11), (false, 1 << 12),
          (false, 1 << 12), (false, 1 << 12), (true, 1 << 12), (false, 1 << 13), (true, 1 << 13),
          (false, 1 << 14), (false, 1 << 14), (false, 1 << 14), (true, 1 << 14), (false, 1 << 15),
          (false, 1 << 15), (false, 1 << 15), (false, 1 << 15), (false, 1 << 15), (true, 1 << 15),
          // 60
          (false, 1 << 16), (true, 1 << 16), (false, 1 << 17), (false, 1 << 17), (false, 1 << 17),
          (false, 1 << 17), (false, 1 << 17), (true, 1 << 17), (false, 1 << 18), (false, 1 << 18),
          (false, 1 << 18), (true, 1 << 18), (false, 1 << 19), (true, 1 << 19), (false, 1 << 20),
          (false, 1 << 20), (false, 1 << 20), (true, 1 << 20), (false, 1 << 21), (true, 1 << 21),
          (false, 1 << 22), (false, 1 << 22), (false, 1 << 22), (true, 1 << 22), (false, 1 << 23),
          (false, 1 << 23), (false, 1 << 23), (false, 1 << 23), (false, 1 << 23), (true, 1 << 23),
          // 90
          (false, 1 << 24), (true, 1 << 24), (false, 1 << 25), (false, 1 << 25), (false, 1 << 25),
          (false, 1 << 25), (false, 1 << 25), (true, 1 << 25), (false, 1 << 26), (false, 1 << 26),
          (false, 1 << 26), (true, 1 << 26), (false, 1 << 27), (true, 1 << 27), (false, 1 << 28),
          (false, 1 << 28), (false, 1 << 28), (true, 1 << 28), (false, 1 << 29), (true, 1 << 29),
          (false, 1 << 30), (false, 1 << 30), (false, 1 << 30), (true, 1 << 30), (false, 1 << 31),
          (false, 1 << 31), (false, 1 << 31), (false, 1 << 31), (false, 1 << 31), (true, 1 << 31),
          // 120
          (false, 1 << 32), (true, 1 << 32), (false, 1 << 33), (false, 1 << 33), (false, 1 << 33),
          (false, 1 << 33), (false, 1 << 33), (true, 1 << 33), (false, 1 << 34), (false, 1 << 34),
          (false, 1 << 34), (true, 1 << 34), (false, 1 << 35), (true, 1 << 35), (false, 1 << 36),
          (false, 1 << 36), (false, 1 << 36), (true, 1 << 36), (false, 1 << 37), (true, 1 << 37),
          (false, 1 << 38), (false, 1 << 38), (false, 1 << 38), (true, 1 << 38), (false, 1 << 39),
          (false, 1 << 39), (false, 1 << 39), (false, 1 << 39), (false, 1 << 39), (true, 1 << 39),
          // 150
          (false, 1 << 40), (true, 1 << 40), (false, 1 << 41), (false, 1 << 41), (false, 1 << 41),
          (false, 1 << 41), (false, 1 << 41), (true, 1 << 41), (false, 1 << 42), (false, 1 << 42),
          (false, 1 << 42), (true, 1 << 42), (false, 1 << 43), (true, 1 << 43), (false, 1 << 44),
          (false, 1 << 44), (false, 1 << 44), (true, 1 << 44), (false, 1 << 45), (true, 1 << 45),
          (false, 1 << 46), (false, 1 << 46), (false, 1 << 46), (true, 1 << 46), (false, 1 << 47),
          (false, 1 << 47), (false, 1 << 47), (false, 1 << 47), (false, 1 << 47), (true, 1 << 47),
          // 180
          (false, 1 << 48), (true, 1 << 48), (false, 1 << 49), (false, 1 << 49), (false, 1 << 49),
          (false, 1 << 49), (false, 1 << 49), (true, 1 << 49), (false, 1 << 50), (false, 1 << 50),
          (false, 1 << 50), (true, 1 << 50), (false, 1 << 51), (true, 1 << 51), (false, 1 << 52),
          (false, 1 << 52), (false, 1 << 52), (true, 1 << 52), (false, 1 << 53), (true, 1 << 53),
          (false, 1 << 54), (false, 1 << 54), (false, 1 << 54), (true, 1 << 54), (false, 1 << 55),
          (false, 1 << 55), (false, 1 << 55), (false, 1 << 55), (false, 1 << 55), (true, 1 << 55),
          // 210
          (false, 1 << 56), (true, 1 << 56), (false, 1 << 57), (false, 1 << 57), (false, 1 << 57),
          (false, 1 << 57), (false, 1 << 57), (true, 1 << 57), (false, 1 << 58), (false, 1 << 58),
          (false, 1 << 58), (true, 1 << 58), (false, 1 << 59), (true, 1 << 59), (false, 1 << 60),
          (false, 1 << 60), (false, 1 << 60), (true, 1 << 60), (false, 1 << 61), (true, 1 << 61),
          (false, 1 << 62), (false, 1 << 62), (false, 1 << 62), (true, 1 << 62), (false, 1 << 63),
          (false, 1 << 63), (false, 1 << 63), (false, 1 << 63), (false, 1 << 63), (true, 1 << 63),
    ];

    let byte = (idx / MODULUS) as usize;
    let byte_idx = POS[(idx % MODULUS) as usize];
    (byte_idx.0, byte, byte_idx.1)
}

/// Get the bit representing the number at the given index in the range.
#[inline]
pub fn get(segment: &[u64], idx: u64) -> bool {
    match index_for(idx) {
        (false, _, _) => false,
        (true, x, y) => (segment[x] & y != 0),
    }
}

/// Set the bit representing the number at the given index in the range to off.
#[inline]
pub fn set_off(segment: &mut [u64], idx: u64) {
    match index_for(idx) {
        (false, _, _) => {}
        (true, x, y) => segment[x] &= !y,
    }
}

/// Set the bit representing the number at the given index in the range to on.
#[inline]
pub fn set_on(segment: &mut [u64], idx: u64) {
    match index_for(idx) {
        (false, _, _) => {}
        (true, x, y) => segment[x] |= y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_small_values() {
        for ix in 0..MODULUS {
            let mut segment = [!0; 1];
            set_off(&mut segment, ix);
            assert_eq!(get(&segment, ix), false);
            set_on(&mut segment, ix);
            let expected = if (ix % 2 == 0) || (ix % 3 == 0) || (ix % 5 == 0) {
                false
            } else {
                true
            };
            assert_eq!(get(&segment, ix), expected);
        }
    }

    #[test]
    fn set_large_values() {
        for ix in 0..MODULUS {
            let mut segment = [!0; 100];
            set_off(&mut segment, ix + 99 * 30);
            assert_eq!(get(&segment, ix + 99 * 30), false);
            set_on(&mut segment, ix + 99 * 30);
            let expected = if (ix % 2 == 0) || (ix % 3 == 0) || (ix % 5 == 0) {
                false
            } else {
                true
            };
            assert_eq!(get(&segment, ix + 99 * 30), expected);
        }
    }
}