mod instruction_generator;

use instruction_generator::{generate_instruction, Instruction, Parameter, Register};

fn main() {
    println!(
        "Hello, world!{}",
        generate_instruction(
            Instruction::Add,
            &[Parameter::Register(Register::V(0)), Parameter::Byte(42)]
        )
    );
}
