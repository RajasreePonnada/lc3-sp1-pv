//! A lc3 vm program to be proven inside sp1.

#![no_main]
sp1_zkvm::entrypoint!(main);
pub const PC_START: u16 = 0x3000;
use lc3::vm::execute_program;
use lc3::vm::vm::VM;
pub fn main() {
    let instructions = sp1_zkvm::io::read::<Vec<u16>>();
    let mut vm = VM::new();
    for i in 0..instructions.len() {
        vm.write_memory(i + (PC_START as usize), instructions[i]);
    }
    vm.registers.pc = PC_START;
    execute_program(&mut vm);
    let registers_out = vm.registers;
    sp1_zkvm::io::write(&registers_out);
}
