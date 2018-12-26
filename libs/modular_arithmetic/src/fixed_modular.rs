//! A struct which behaves like a number constrained by a fixed modulus.

use crate::internals::{_mod_add, _mod_sub, _mod_mul, _mod_inverse, _normalise};
use numeric_traits::{Zero, One};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul};

/// Until const-generics are available, use types implementing this trait as a stand-in for
/// an associated const on the `FixedModular` struct.
pub trait Modulus: Clone + Copy + Debug + PartialEq + Eq {
    fn modulus() -> u64;
}

/// A utility macro used to easily define a struct which implements `Modulus`
#[macro_export]
macro_rules! define_modulus {
    ($t:ident, $m:ident, $mod:expr) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct $m {}

        impl Modulus for $m {
            fn modulus() -> u64 {
                $mod
            }
        }

        type $t = FixedModular<$m>;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedModular<T: Modulus> {
    pub value: u64,
    modulus: PhantomData<T>,
}

impl<T: Modulus> FixedModular<T> {

    pub fn inverse(&self) -> Option<FixedModular<T>> {
        _mod_inverse(self.value, T::modulus()).map(|x| Self::from(x))
    }
}

impl<T: Modulus> From<u64> for FixedModular<T> {
    fn from(value: u64) -> FixedModular<T> {
        FixedModular {
            value: _normalise(value, T::modulus()),
            modulus: PhantomData,
        }
    }
}

impl<T: Modulus> Into<u64> for FixedModular<T> {
    fn into(self) -> u64 {
        self.value
    }
}

impl<T: Modulus> Zero for FixedModular<T> {

    fn zero() -> FixedModular<T> {
        Self::from(0)
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl<T: Modulus> One for FixedModular<T> {

    fn one() -> FixedModular<T> {
        Self::from(1)
    }

    fn is_one(&self) -> bool {
        self.value == 1
    }
}

impl<T: Modulus> Add for FixedModular<T> {
    type Output = FixedModular<T>;

    fn add(self, rhs: FixedModular<T>) -> FixedModular<T> {
        Self::from(_mod_add(self.value, rhs.value, T::modulus()))
    }
}

impl<T: Modulus> Sub for FixedModular<T> {
    type Output = FixedModular<T>;

    fn sub(self, rhs: FixedModular<T>) -> FixedModular<T> {
        Self::from(_mod_sub(self.value, rhs.value, T::modulus()))
    }
}

impl<T: Modulus> Mul for FixedModular<T> {
    type Output = FixedModular<T>;

    fn mul(self, rhs: FixedModular<T>) -> FixedModular<T> {
        Self::from(_mod_mul(self.value, rhs.value, T::modulus()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    define_modulus!(Mod2, M2, 2);
    define_modulus!(Mod13, M13, 13);
    define_modulus!(Mod101, M101, 101);

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