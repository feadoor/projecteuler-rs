//! A macro that defines a struct which behaves like a number constrained by a given modulus.

use numeric_traits::{Zero, One};
use std::ops::{Add, Sub, Mul};
use functions::{mod_add, mod_sub, mod_mul, mod_inverse};

/// Until const-generics are available, this macro is used to define an ad-hoc type for any given
/// modulus - long-term, a struct with a constant generic parameter makes more sense.
macro_rules! define_modulus {
    ($t: ident, $mod: expr) => {

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct $t {
            pub value: u64,
        }

        impl $t {
            pub fn inverse(&self) -> Option<$t> {
                mod_inverse(self.value, $mod).map(|x| $t { value: x })
            }
        }

        impl From<u64> for $t {
            fn from(value: u64) -> $t {
                $t { value: value % $mod }
            }
        }

        impl Into<u64> for $t {
            fn into(self) -> u64 {
                self.value
            }
        }

        impl Zero for $t {
            fn zero() -> $t {
                $t { value: 0 }
            }

            fn is_zero(&self) -> bool {
                self.value == 0
            }
        }

        impl One for $t {
            fn one() -> $t {
                $t { value: 1 }
            }

            fn is_one(&self) -> bool {
                self.value == 1
            }
        }

        impl Add for $t {
            type Output = $t;

            fn add(self, rhs: $t) -> $t {
                $t { value: mod_add(self.value, rhs.value, $mod) }
            }
        }

        impl Sub for $t {
            type Output = $t;

            fn sub(self, rhs: $t) -> $t {
                $t { value: mod_sub(self.value, rhs.value, $mod) }
            }
        }

        impl Mul for $t {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                $t { value: mod_mul(self.value, rhs.value, $mod) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    define_modulus!(Mod2, 2);
    define_modulus!(Mod13, 13);
    define_modulus!(Mod101, 101);

    #[test]
    fn test_definition() {
        assert_eq!(Mod2::from(0).value, 0);
        assert_eq!(Mod2::from(1).value, 1);
        assert_eq!(Mod2::from(7).value, 1);

        assert_eq!(Mod13::from(0).value, 0);
        assert_eq!(Mod13::from(6).value, 6);
        assert_eq!(Mod13::from(15).value, 2);

        assert_eq!(Mod101::from(0).value, 0);
        assert_eq!(Mod101::from(45).value, 45);
        assert_eq!(Mod101::from(306).value, 3);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Mod2::from(0) + Mod2::from(0), Mod2::from(0));
        assert_eq!(Mod2::from(1) + Mod2::from(1), Mod2::from(0));
        assert_eq!(Mod2::from(0) + Mod2::from(1), Mod2::from(1));

        assert_eq!(Mod13::from(5) + Mod13::from(6), Mod13::from(11));
        assert_eq!(Mod13::from(8) + Mod13::from(9), Mod13::from(4));

        assert_eq!(Mod101::from(50) + Mod101::from(51), Mod101::from(0));
        assert_eq!(Mod101::from(2) + Mod101::from(100), Mod101::from(1));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Mod2::from(0) - Mod2::from(0), Mod2::from(0));
        assert_eq!(Mod2::from(1) - Mod2::from(1), Mod2::from(0));
        assert_eq!(Mod2::from(0) - Mod2::from(1), Mod2::from(1));
        assert_eq!(Mod2::from(1) - Mod2::from(0), Mod2::from(1));

        assert_eq!(Mod13::from(5) - Mod13::from(6), Mod13::from(12));
        assert_eq!(Mod13::from(6) - Mod13::from(5), Mod13::from(1));
        assert_eq!(Mod13::from(4) - Mod13::from(7), Mod13::from(10));
        assert_eq!(Mod13::from(7) - Mod13::from(4), Mod13::from(3));

        assert_eq!(Mod101::from(20) - Mod101::from(77), Mod101::from(44));
        assert_eq!(Mod101::from(77) - Mod101::from(20), Mod101::from(57));
        assert_eq!(Mod101::from(2) - Mod101::from(100), Mod101::from(3));
        assert_eq!(Mod101::from(100) - Mod101::from(2), Mod101::from(98));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Mod2::from(0) * Mod2::from(0), Mod2::from(0));
        assert_eq!(Mod2::from(1) * Mod2::from(1), Mod2::from(1));
        assert_eq!(Mod2::from(0) * Mod2::from(1), Mod2::from(0));

        assert_eq!(Mod13::from(5) * Mod13::from(6), Mod13::from(4));
        assert_eq!(Mod13::from(8) * Mod13::from(9), Mod13::from(7));

        assert_eq!(Mod101::from(50) * Mod101::from(51), Mod101::from(25));
        assert_eq!(Mod101::from(2) * Mod101::from(100), Mod101::from(99));
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Mod2::from(0).inverse(), None);
        assert_eq!(Mod2::from(1).inverse(), Some(Mod2::from(1)));

        assert_eq!(Mod13::from(4).inverse(), Some(Mod13::from(10)));
        assert_eq!(Mod13::from(5).inverse(), Some(Mod13::from(8)));

        assert_eq!(Mod101::from(2).inverse(), Some(Mod101::from(51)));
        assert_eq!(Mod101::from(47).inverse(), Some(Mod101::from(43)));
    }
}