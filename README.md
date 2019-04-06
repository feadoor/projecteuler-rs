# projecteuler-rs [![Build Status](https://travis-ci.org/feadoor/projecteuler-rs.svg?branch=master)](https://travis-ci.org/feadoor/projecteuler-rs)

Solutions to [Project Euler](https://projecteuler.net) problems in Rust.

## Usage

Build and run with the following command to run the solution to a particular problem:

```
cargo run --release --bin problemXXX
```

There is a single test for each problem, simply checking that the answer is correct. To run the tests:

```
cargo test --release
```

There are also several library crates under `/libs` which are used in multiple problems. To build and test one of these crates:

```
cargo test --release -p <crate>
```

## Documentation

The [documentation](https://feadoor.github.io/projecteuler-rs) explains, at a high level, the solution to each solved problem.

There is also documentation for each of the library crates used in the solutions - this can be found at the same link.

## Solution times

Project Euler maintains that each problem can be solved in under a minute with the right algorithm. To time each solution, and check if any
take more than 60 seconds to complete (Python 2 required):

```
cargo build --release
python timings-test.py target/release/projecteuler-rs
```
