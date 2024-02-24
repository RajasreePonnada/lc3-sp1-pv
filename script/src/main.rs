//! A simple script to generate and verify the proof of a given program.

use lc3::vm::registers::Registers;
use sp1_core::{SP1Prover, SP1Stdin, SP1Verifier};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    // instruction set to compute square of 7
    let instructions: Vec<u16> = vec![
        0b0001_010_001_1_00111, // ADD R2, R1, #7  ; Add 7 to R1, all registers are initialized to 0
        0b0001010010100111,
        0b0001010010100111,
        0b0001010010100111,
        0b0001010010100111,
        0b0001010010100111,
        0b0001010010100111,
    ];
    // result is stored in R2
    stdin.write(&instructions);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let registers = proof.stdout.read::<Registers>();
    println!("registers: {:?}", registers);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
