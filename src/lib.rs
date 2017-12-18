//! Solutions in Rust to the problems at [Project Euler](https://projecteuler.net)
//!
//! Each solved problem has a high-level description of the solution in its documentation.

extern crate ansi_term;

use ansi_term::Colour::{Cyan, Green};
use ansi_term::Style;

/// Run a particular problem, printing its solution to the terminal.
pub fn exec(problem: fn() -> String) {
    println!("{}", Green.bold().paint("Running..."));
    println!("{} - {}",
             Cyan.bold().paint("Solution"),
             Style::new().bold().paint(problem()));
}

#[macro_export]
macro_rules! problem {
    ($solver:ident, $answer:expr) => (
        fn main() {
            projecteuler_rs::exec($solver);
        }

        #[cfg(test)]
        mod tests {
            use super:: *;

            #[test]
            fn test() {
                assert_eq!($solver(), $answer);
            }
        }
    );
}
