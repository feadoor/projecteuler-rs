//! Traits for operations on numeric types.

use std::ops::{Add, Sub, Mul, Div, Rem};

/// Defines an additive identity for `Self`
pub trait Zero: Sized + Add<Self, Output = Self> {
    /// Returns the additive identity element of `Self`
    fn zero() -> Self;
    /// Returns `true` if `self` is equal to the additive identity
    fn is_zero(&self) -> bool;
}

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            fn zero() -> $t { $v }
            fn is_zero(&self) -> bool { *self == $v }
        }
    }
}

zero_impl!(usize, 0usize);
zero_impl!(u8,    0u8);
zero_impl!(u16,   0u16);
zero_impl!(u32,   0u32);
zero_impl!(u64,   0u64);

zero_impl!(isize, 0isize);
zero_impl!(i8,    0i8);
zero_impl!(i16,   0i16);
zero_impl!(i32,   0i32);
zero_impl!(i64,   0i64);

zero_impl!(f32, 0.0f32);
zero_impl!(f64, 0.0f64);

/// Defines a multiplicative identity for `Self`
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`
    fn one() -> Self;
    /// Returns `true` if `self` is equal to the multiplicative identity
    fn is_one(&self) -> bool;
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            fn one() -> $t { $v }
            fn is_one(&self) -> bool { *self == $v }
        }
    }
}

one_impl!(usize, 1usize);
one_impl!(u8,    1u8);
one_impl!(u16,   1u16);
one_impl!(u32,   1u32);
one_impl!(u64,   1u64);

one_impl!(isize, 1isize);
one_impl!(i8,    1i8);
one_impl!(i16,   1i16);
one_impl!(i32,   1i32);
one_impl!(i64,   1i64);

one_impl!(f32, 1.0f32);
one_impl!(f64, 1.0f64);

/// A trait for numeric types which allow addition, subtraction and multiplication.
pub trait Algebraic: Zero + One + Add<Self, Output = Self> + Sub<Self, Output = Self> +
                     Mul<Self, Output = Self> + PartialEq<Self>
{ }

impl<T> Algebraic for T
where T: Zero + One + Add<Self, Output = Self> + Sub<Self, Output = Self> +
         Mul<Self, Output = Self> + PartialEq<Self>
{ }

/// A trait for numeric types which allow division and remainder operations.
pub trait DivRem: Sized + Div<Self, Output = Self> + Rem<Self, Output = Self> { }

impl<T> DivRem for T
where T: Div<Self, Output = Self> + Rem<Self, Output = Self>
{ }

/// A trait for numeric types which support saturating operations.
pub trait Saturating {
    /// Saturating addition operator
    fn saturating_add(self, v: Self) -> Self;

    /// Saturating subtraction operator
    fn saturating_sub(self, v: Self) -> Self;

    /// Saturating multiplication operator
    fn saturating_mul(self, v: Self) -> Self;
}

macro_rules! saturating_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            fn saturating_add(self, v: Self) -> Self {
                Self::saturating_add(self, v)
            }

            fn saturating_sub(self, v: Self) -> Self {
                Self::saturating_sub(self, v)
            }

            fn saturating_mul(self, v: Self) -> Self {
                Self::saturating_mul(self, v)
            }
        }
    )*}
}

saturating_impl!(Saturating for isize usize i8 u8 i16 u16 i32 u32 i64 u64);

/// A trait for numeric types which support checked operations.
pub trait Checked: Sized {
    /// Checked addition operator
    fn checked_add(self, v: Self) -> Option<Self>;

    /// Checked subtraction operator
    fn checked_sub(self, v: Self) -> Option<Self>;

    /// Checked multiplication operator
    fn checked_mul(self, v: Self) -> Option<Self>;
}

macro_rules! checked_impl {
    ($trait_name:ident for $($t:ty)*) => {$(
        impl $trait_name for $t {
            fn checked_add(self, v: Self) -> Option<Self> {
                Self::checked_add(self, v)
            }

            fn checked_sub(self, v: Self) -> Option<Self> {
                Self::checked_sub(self, v)
            }

            fn checked_mul(self, v: Self) -> Option<Self> {
                Self::checked_mul(self, v)
            }
        }
    )*}
}

checked_impl!(Checked for isize usize i8 u8 i16 u16 i32 u32 i64 u64);
