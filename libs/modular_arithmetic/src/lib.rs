//! Functions and structs relating to modular arithmetic.

extern crate number_theory;
extern crate numeric_traits;

mod crt;
mod functions;
mod modular;

pub use crt::*;
pub use functions::*;
pub use modular::*;
