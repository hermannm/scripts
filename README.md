# scripts

Utility scripts written in Rust.

## Project structure

Each `.rs` file in `src/bin` is a script. `src/lib.rs` and surrounding modules provide shared
utility code.

## Running scripts

1. Install Rust: https://www.rust-lang.org/tools/install
2. From within this project, run `cargo run --bin <script-name>`, where `<script-name>` is the name
   of one of the scripts under `src/bin`
   - For example, to run `src/bin/gen-uuid.rs`, type `cargo run --bin gen-uuid`
   - Add the `--release` flag to make it run faster (at the cost of a little longer compilation time
     the first time it's run)
   - To compile scripts to reusable binaries: run `cargo build --release`
     - Binaries will then be located under `target/release`
