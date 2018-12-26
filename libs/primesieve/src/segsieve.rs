//! An implementation of the segmented sieve of Eratosthenes.

use std::cmp::min;
use std::slice::from_raw_parts_mut;

use crate::iterator::SieveIterator;
use crate::segment::set_off;
use crate::wheel::Wheel30;

const MODULUS: u64 = 240;
const SEGMENT_LEN: usize = 32768;
const SEGMENT_SIZE: u64 = MODULUS * SEGMENT_LEN as u64;

/// Returns a sequence of `u64`s encoding the primes up to the square root of the given limit, but
/// excluding 2, 3 and 5.
fn small_primes(limit: u64) -> Vec<u64> {
    // Start by allocating enough `u64`s to hold information about the numbers up to the required
    // square root.
    let sqrt = (limit as f64).sqrt() as u64;
    let mut sieve = vec![!0; (sqrt / MODULUS + 1) as usize];
    let small_limit = 240 * sieve.len() as u64;

    // Correct the first entry of the sieve to only contain 1's in positions corresponding to true
    // prime numbers - this just speeds things up a little as it prevents the iterator from
    // accidentally considering non-primes early in its life.
    sieve[0] = 0b1111100100111101110110111011011001111110111011111101111111111110;

    // Iterate over the prime numbers held in the sieve and cross of multiples of each one.
    // Since we cannot usually have a mutable borrow and an immutable borrow to the sieve at the
    // same time, there's some unsafe code here to do just that, and we'll make a promise to the
    // compiler that we're not doing anything nasty 😮
    unsafe {
        let sieve_mut = from_raw_parts_mut(sieve.as_mut_ptr(), sieve.len());
        let iter = SieveIterator::new(&sieve);
        for prime in iter {
            // For each prime p, we cross off the multiples of it larger than p^2 which are not
            // multiples of 2, 3 or 5.
            let mut wheel = Wheel30::new(prime, prime);
            let mut multiple = prime * prime;
            if multiple >= small_limit {
                break;
            }
            while multiple < small_limit {
                set_off(sieve_mut, multiple);
                multiple += wheel.next_diff();
            }
        }
    }

    sieve
}

/// Sieve primes up to the given limit using a segmented sieve of Eratosthenes, and return a
/// vector of `u64`s encoding the primes.
pub fn segmented_sieve(limit: u64) -> Vec<u64> {
    // First, we need to sieve the primes up to the square root of the given limit - these will be
    // the primes whose multiples are crossed off the sieve.
    let lim = limit + MODULUS - (limit % MODULUS);
    let small_primes = small_primes(lim);
    let mut small_primes_iter = SieveIterator::new(&small_primes);

    // Here's the array in which we'll do our sieving of the segments, and a vector in which we'll
    // store the final results.
    let mut segment = [!0; SEGMENT_LEN];
    segment[0] ^= 1;
    let mut segments = Vec::with_capacity((lim / MODULUS) as usize);

    // Here are the indices into the segment for the next multiple of each prime whose multiples
    // are being crossed off - the first entry is the index, and the second entry is a wheel which
    // generates the differences between successive indices.
    let mut next_indices = Vec::<(u64, Wheel30)>::new();

    // Iterate over segments for as long as we still have more sieving to do.
    let mut low = 0;
    while low <= lim {

        // Now, add the new sieving primes which we will need for this segment.
        let high = min(low + SEGMENT_SIZE, lim);
        let segment_size = high - low;

        while let Some(prime) = small_primes_iter.next() {
            next_indices.push((prime * prime - low, Wheel30::new(prime, prime)));
            if prime * prime >= high {
                break;
            }
        }

        // Sieve the current segment
        for &mut (ref mut index, ref mut wheel) in &mut next_indices {
            while *index < segment_size {
                set_off(&mut segment, *index);
                *index += wheel.next_diff();
            }
            *index -= segment_size;
        }

        // Store the result of this pass and prepare for the next pass.
        segments.extend_from_slice(
            if segment_size < SEGMENT_SIZE {
                &segment[..(segment_size / MODULUS) as usize]
            } else {
                &segment
            }
        );

        low += SEGMENT_SIZE;
        segment = [!0; SEGMENT_LEN];
    }

    segments
}

#[test]
fn test_small_primes() {
    let sieve = small_primes(1000000);
    let primes = SieveIterator::new(&sieve).collect::<Vec<u64>>();
    assert_eq!(primes,
               vec![7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                    83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                    163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
                    241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331,
                    337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
                    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509,
                    521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613,
                    617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709,
                    719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821,
                    823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
                    929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019,
                    1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093,
                    1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187,
                    1193]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use iterator::SieveIterator;

    #[test]
    fn test_small_segmented_sieve() {
        let sieve = segmented_sieve(1000);
        let primes = SieveIterator::new(&sieve).collect::<Vec<u64>>();
        assert_eq!(primes,
                   vec![7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                        83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                        163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
                        241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331,
                        337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
                        431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509,
                        521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613,
                        617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709,
                        719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821,
                        823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
                        929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019,
                        1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093,
                        1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187,
                        1193]);
    }

    #[test]
    fn test_large_segmented_sieve() {
        let sieve = segmented_sieve(50000000);
        let primes = SieveIterator::new(&sieve).collect::<Vec<u64>>();
        assert_eq!(primes[primes.len() - 100..].to_vec(),
                   vec![49998539, 49998563, 49998587, 49998593, 49998601, 49998617, 49998623,
                        49998653, 49998659, 49998661, 49998727, 49998743, 49998749, 49998763,
                        49998779, 49998791, 49998811, 49998821, 49998827, 49998841, 49998857,
                        49998869, 49998911, 49998913, 49998917, 49998919, 49998931, 49998947,
                        49998953, 49998983, 49999031, 49999069, 49999111, 49999121, 49999133,
                        49999151, 49999177, 49999207, 49999231, 49999253, 49999267, 49999289,
                        49999291, 49999297, 49999307, 49999349, 49999351, 49999361, 49999387,
                        49999403, 49999409, 49999423, 49999427, 49999441, 49999463, 49999471,
                        49999489, 49999529, 49999553, 49999561, 49999589, 49999597, 49999603,
                        49999613, 49999619, 49999627, 49999637, 49999639, 49999643, 49999667,
                        49999673, 49999693, 49999699, 49999711, 49999739, 49999751, 49999753,
                        49999757, 49999759, 49999777, 49999783, 49999801, 49999819, 49999843,
                        49999847, 49999853, 49999877, 49999883, 49999897, 49999903, 49999921,
                        49999991, 50000017, 50000021, 50000047, 50000059, 50000063, 50000101,
                        50000131, 50000141]);
    }
}