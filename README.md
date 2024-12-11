# advent-of-code-2024
advent of code solutions for 2024

# Usage

files can be fetched with a python script

`python fetch_file.py`

- `cargo run -- {day} {part}` for running a single solution
- `cargo bench --features all-benches` for benchmarking all solutions
- `cargo bench bench{day} --features all-benches` for benchmarking a single day

## nix
cargo is expected to be preinstalled on the system

a `shell.nix` exist for python
