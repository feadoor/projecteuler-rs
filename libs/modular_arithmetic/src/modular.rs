//! A macro that defines a struct which behaves like a number constrained by a given modulus.

/// Until const-generics are available, this macro is used to define an ad-hoc type for any given
/// modulus - long-term, a struct with a constant generic parameter makes more sense.
macro_rules! define_modulus {
    ($t: ident, $e: expr) => {
        struct $t {
            value: u64;
        }
    }
}