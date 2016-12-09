//! Solutions in Rust to the problems at [Project Euler](https://projecteuler.net)
//!
//! Each solved problem has a high-level description of the solution in its documentation.

// Other people's crates
extern crate ansi_term;
extern crate docopt;
#[macro_use]
extern crate itertools;
extern crate num;
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
        6 => run!(problem006),
        7 => run!(problem007),
        8 => run!(problem008),
        9 => run!(problem009),
        10 => run!(problem010),
        11 => run!(problem011),
        12 => run!(problem012),
        13 => run!(problem013),
        14 => run!(problem014),
        15 => run!(problem015),
        16 => run!(problem016),
        17 => run!(problem017),
        18 => run!(problem018),
        19 => run!(problem019),
        20 => run!(problem020),
        21 => run!(problem021),
        22 => run!(problem022),
        23 => run!(problem023),
        24 => run!(problem024),
        25 => run!(problem025),
        26 => run!(problem026),
        27 => run!(problem027),
        28 => run!(problem028),
        29 => run!(problem029),
        30 => run!(problem030),
        31 => run!(problem031),
        32 => run!(problem032),
        33 => run!(problem033),
        34 => run!(problem034),
        35 => run!(problem035),
        36 => run!(problem036),
        37 => run!(problem037),
        38 => run!(problem038),
        n => println!("Problem {} has not yet been solved", n),
    }
}
