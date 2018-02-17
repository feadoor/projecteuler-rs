//! Simple functions dealing with modular arithmetic

use std::cmp::Ordering;
use std::mem::swap;

#[inline(always)]
pub fn _normalise(a: u64, m: u64) -> u64 {
    match a.cmp(&m) {
        Ordering::Greater => a % m,
        Ordering::Equal => 0,
        Ordering::Less => a,
    }
}

#[inline(always)]
pub fn _mod_add(a: u64, b: u64, m: u64) -> u64 {
    match a + b {
        x if x > m => x - m,
        x => x,
    }
}

#[inline(always)]
pub fn _mod_sub(a: u64, b: u64, m: u64) -> u64 {
    match a.cmp(&b) {
        Ordering::Less => a + m - b,
        Ordering::Equal => 0,
        Ordering::Greater => a - b,
    }
}

#[inline(always)]
pub fn _mod_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    match a.checked_mul(b) {
        Some(x) => _normalise(x, m),
        None => {
            if a > b { swap(&mut a, &mut b); }
            let mut result = 0;

            while a > 0 {
                if a & 1 != 0 { result = _mod_add(result, b, m); }
                a >>= 1;
                b = _mod_add(b, b, m);
            }

            result
        }
    }
}

#[inline(always)]
pub fn _mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (mut u1, mut u3) = (1, a);
    let (mut v1, mut v3) = (0, m);
    let mut odd_iterations = false;

    while v3 != 0 {
        let q = u3 / v3;

        u1 = u1 + q * v1;
        u3 = u3 - q * v3;
        swap(&mut u1, &mut v1);
        swap(&mut u3, &mut v3);

        odd_iterations = !odd_iterations;
    }

    if u3 == 1 {
        if odd_iterations { Some(m - u1) } else { Some(u1) }
    } else {
        None
    }
}
