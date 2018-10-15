# Rubix-Cube
Rust- Multithreaded brute force Rubix Cube solver

Finds the minimal number of moves to solve a Rubix cube utilizing a Spanning Tree.

The notation used is https://solvethecube.com/notation

Requirements: Rust tool chain. Install with
```
curl https://sh.rustup.rs -sSf | sh
```

To run: From base directory of project:

```
cargo build --release
time target/release/cube
```
