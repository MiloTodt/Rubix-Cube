# Rubix-Cube
## Multithreaded recursive brute force Rubix Cube solver

Finds the minimal number of moves to solve a Rubix cube utilizing a binomial heap.

Only use of non-standard library is a single call to generate a random number.

The notation used is https://solvethecube.com/notation

## Requirements 
Rust tool chain. Install with
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## To run
From base directory of project

```
cargo build --release
time target/release/cube
```
## Example Outputs

![Output 1](https://i.imgur.com/19jNDp7.png)
![Output 2](https://i.imgur.com/cCKYDHl.png)
![Output 3](https://i.imgur.com/EGhITsJ.png)
