//! Solutions in Rust to the problems at [Project Euler](https://projecteuler.net)
//!
//! Each solved problem has a high-level description of the solution in its documentation.

// Other people's crates
extern crate ansi_term;
extern crate docopt;
extern crate itertools;
extern crate rustc_serialize;

// My own crates
extern crate number_theory;
extern crate primesieve;

pub mod problems;

use ansi_term::Colour::{Cyan, Green};
use ansi_term::Style;
use docopt::Docopt;

use problems::*;

const USAGE: &'static str = "
Usage: projecteuler-rs run <problem>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_problem: u32,
}

/// Run a particular problem, printing its solution to the terminal.
fn exec(name: &str, desc: &str, problem: fn() -> String) {
    let long_desc = format!("Running {} ({})", name, desc);
    println!("{}", Green.bold().paint(long_desc));
    println!("{} - {}",
             Cyan.bold().paint("Solution"),
             Style::new().bold().paint(problem()));
}

macro_rules! run {
    ($e:ident) => {
        exec($e::NAME, $e::DESC, $e::answer)
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match args.arg_problem {
        1 => run!(problem001),
        2 => run!(problem002),
        3 => run!(problem003),
        4 => run!(problem004),
        5 => run!(problem005),
        n => println!("Problem {} has not yet been solved", n),
    }
}
