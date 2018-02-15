//! A struct which behaves like a number constrained by a variable modulus.

use std::ops::{Add, Sub, Mul};
use functions::{mod_add, mod_sub, mod_mul, mod_inverse};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Modular {
    value: u64,
    modulus: u64,
}

impl Modular {
    pub fn new(value: u64, modulus: u64) -> Modular {
        Modular {
            value: value % modulus,
            modulus: modulus,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    pub fn inverse(&self) -> Option<Modular> {
        mod_inverse(self.value, self.modulus).map(|x| Modular::new(x, self.modulus))
    }
}

impl Into<u64> for Modular {
    fn into(self) -> u64 {
        self.value
    }
}

impl Add for Modular {
    type Output = Modular;

    fn add(self, rhs: Modular) -> Modular {
        assert_eq!(self.modulus, rhs.modulus);
        Modular::new(mod_add(self.value, rhs.value, self.modulus), self.modulus)
    }
}

impl Sub for Modular {
    type Output = Modular;

    fn sub(self, rhs: Modular) -> Modular {
        assert_eq!(self.modulus, rhs.modulus);
        Modular::new(mod_sub(self.value, rhs.value, self.modulus), self.modulus)
    }
}

impl Mul for Modular {
    type Output = Modular;

    fn mul(self, rhs: Modular) -> Modular {
        assert_eq!(self.modulus, rhs.modulus);

        Modular::new(mod_mul(self.value, rhs.value, self.modulus), self.modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! modular {
        ($v:expr, $m:expr) => {
            Modular::new($v, $m)
        }
    }

    #[test]
    fn test_definition() {
        assert_eq!(modular!(0, 2).value(), 0);
        assert_eq!(modular!(1, 2).value(), 1);
        assert_eq!(modular!(7, 2).value(), 1);

        assert_eq!(modular!(0, 13).value(), 0);
        assert_eq!(modular!(6, 13).value(), 6);
        assert_eq!(modular!(15, 13).value(), 2);

        assert_eq!(modular!(0, 101).value(), 0);
        assert_eq!(modular!(45, 101).value(), 45);
        assert_eq!(modular!(306, 101).value(), 3);
    }

    #[test]
    fn test_addition() {
        assert_eq!(modular!(0, 2) + modular!(0, 2), modular!(0, 2));
        assert_eq!(modular!(1, 2) + modular!(1, 2), modular!(0, 2));
        assert_eq!(modular!(0, 2) + modular!(1, 2), modular!(1, 2));

        assert_eq!(modular!(5, 13) + modular!(6, 13), modular!(11, 13));
        assert_eq!(modular!(8, 13) + modular!(9, 13), modular!(4, 13));

        assert_eq!(modular!(50, 101) + modular!(51, 101), modular!(0, 101));
        assert_eq!(modular!(2, 101) + modular!(100, 101), modular!(1, 101));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(modular!(0, 2) - modular!(0, 2), modular!(0, 2));
        assert_eq!(modular!(1, 2) - modular!(1, 2), modular!(0, 2));
        assert_eq!(modular!(0, 2) - modular!(1, 2), modular!(1, 2));
        assert_eq!(modular!(1, 2) - modular!(0, 2), modular!(1, 2));

        assert_eq!(modular!(5, 13) - modular!(6, 13), modular!(12, 13));
        assert_eq!(modular!(6, 13) - modular!(5, 13), modular!(1, 13));
        assert_eq!(modular!(4, 13) - modular!(7, 13), modular!(10, 13));
        assert_eq!(modular!(7, 13) - modular!(4, 13), modular!(3, 13));

        assert_eq!(modular!(20, 101) - modular!(77, 101), modular!(44, 101));
        assert_eq!(modular!(77, 101) - modular!(20, 101), modular!(57, 101));
        assert_eq!(modular!(2, 101) - modular!(100, 101), modular!(3, 101));
        assert_eq!(modular!(100, 101) - modular!(2, 101), modular!(98, 101));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(modular!(0, 2) * modular!(0, 2), modular!(0, 2));
        assert_eq!(modular!(1, 2) * modular!(1, 2), modular!(1, 2));
        assert_eq!(modular!(0, 2) * modular!(1, 2), modular!(0, 2));

        assert_eq!(modular!(5, 13) * modular!(6, 13), modular!(4, 13));
        assert_eq!(modular!(8, 13) * modular!(9, 13), modular!(7, 13));

        assert_eq!(modular!(50, 101) * modular!(51, 101), modular!(25, 101));
        assert_eq!(modular!(2, 101) * modular!(100, 101), modular!(99, 101));
    }

    #[test]
    fn test_inverse() {
        assert_eq!(modular!(0, 2).inverse(), None);
        assert_eq!(modular!(1, 2).inverse(), Some(modular!(1, 2)));

        assert_eq!(modular!(4, 13).inverse(), Some(modular!(10, 13)));
        assert_eq!(modular!(5, 13).inverse(), Some(modular!(8, 13)));

        assert_eq!(modular!(2, 101).inverse(), Some(modular!(51, 101)));
        assert_eq!(modular!(47, 101).inverse(), Some(modular!(43, 101)));
    }
}