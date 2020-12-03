# Advent of Code 2020 Solutions using Rust

This repository is both a boilerplate for building solutions for the [Advent of Code 2020](https://adventofcode.com/2020), as well as a set of solutions which are located on a [solutions](https://github.com/iferc/advent-of-code-2020/tree/solutions) branch. The solutions are kept on another branch to leave this repository open for others use this to attempt their own solutions without spoilers.

Copies of the questions and inputs will be kept in this repository for posterity. Note that all questions are from the awesome work that the folks who put together the [Advent of Code 2020](https://adventofcode.com/2020) have done. My contribution is this boilerplate for solving with [Rust](https://www.rust-lang.org/). The inputs were generated to my login, but if you login to the site yourself you will be given a different set of inputs.

All source code in this repository is written in [Rust](https://www.rust-lang.org/), and structured as a workspace of two modules:
- a `challenges` library which contains implementations of solvers to the challenge problems; and
- a `runner` cli application which can run specific challenge dates and return results along with timing statistics

To implement your own solutions, modify code under the `challenges` crate. Each day's challenge is in a folder of it's own name, containing a default `mod.rs` file that will get executed by the `runner` with the default input data from the `input` directory. The `runner` should not need to be modified, and will only either run days where an input file exists, or an explicit day and data is given via cli.

The output for a single day usually looks like this:
```
===> Day 1 <===
-> Input data <-
Processing time: 16618 ns
-> Silver <-
Processing time: 2301 ns
Ok(
    SilverSolution {
        numbers: [
            661,
            1359,
        ],
        result: 898299,
    },
)
-> Gold <-
Processing time: 53194 ns
Ok(
    GoldSolution {
        numbers: [
            297,
            354,
            1369,
        ],
        result: 143933922,
    },
)
```

# Quick Start

## Install Rust Toolchain

Recommended method for install rust toolchain is with [rustup](https://rustup.rs/). There are shell commands and release downloads available.

Recommended extensions to the toolchain:
```sh
# provides `cargo add <package>` and `cargo remove <package>`
# otherwise requires manual editing of Cargo.toml
cargo install cargo-edit

# provides ability to run commands on project file changes, respects .gitignore
# `cargo watch -x run` will auto-rebuild and run the application
# `cargo watch -s <expr>` will run shell commands
cargo install cargo-watch

# optional for learning urposes
# can be used to see precompiled source before final compilation
cargo install cargo-expand
```

## Solving Challenges

```sh
# build an unoptimized debug executable
cargo build

# run the unoptimized debug executable; will auto build first
cargo run

# auto rebuild and run on file change during development
cargo watch -x run

# build an optimized release executable
cargo build --release

# run the optimized release executable
./target/release/advent-of-code-2020
```

For available options
```sh
# to see all available options
cargo run -- --help
# or
./target/release/advent-of-code-2020 --help
```

All builds compile the the `./target` directory.
