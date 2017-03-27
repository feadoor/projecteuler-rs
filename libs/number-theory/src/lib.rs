//! Some miscellaneous functions of a number-theoretic flavour.

extern crate num_traits;
extern crate rand;

mod iterators;
mod misc;
mod modular;

pub use iterators::*;
pub use misc::*;
pub use modular::*;
