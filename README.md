# LC3 Zero-Knowledge Prover & Verifier with SP1 VM

This repository contains implementation of a zero-knowledge prover and verifier tailored for an arbitrary set of LC3 compatible instructions, leveraging the SP1 Zero Knowledge Virtual Machine.

## Usage

### Prerequisites

Before getting started, ensure that Rust is installed on your system. You can install Rust by following the [official installation guide](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

Additionally, for SP1 installation, follow the [SP1 installation guide](https://succinctlabs.github.io/sp1/getting-started/install.html).

### Build Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/surajj3404/lc3-sp1-pv.git
   cd lc3-sp1-pv

2. Build Program

    ```bash
    cd program
    cargo prove build

### Genarating proofs

1. In `./scripts/src/main.rs`, add the LC3 instructions for which you want to generate proofs. By default, the instruction set is configured to compute the square of 7.

    ```rust
    let instructions: Vec<u16> = vec![/* Add your LC3 instructions here */];

2. Once you have modified the instruction you can genarate proofs by running the following commands. The proof genarated will be stored as a json file in the script directory.

    ```bash
    cd ./script
    cargo run

## Links to Repositories

- [LC3 Virtual Machine Repository](https://github.com/manojkgorle/lc3)

## Contributions

Contributions, bug reports, and feature requests are welcome! Fork the repository, make your changes, and submit a pull request.
