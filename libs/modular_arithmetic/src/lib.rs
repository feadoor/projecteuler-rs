//! Functions and structs relating to modular arithmetic.

extern crate number_theory;
extern crate numeric_traits;

mod crt;
mod fixed_modular;
mod functions;
mod internals;

pub use crt::*;
pub use fixed_modular::*;
pub use functions::*;
