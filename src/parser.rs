use crate::instruction_generator::{generate_instruction, Instruction, Parameter, Register};

pub fn parse(text: &str) -> Vec<u16> {
    let mut binary = Vec::<u16>::new();

    for line in text.lines() {
        if let Some(bin) = parse_line(line) {
            binary.push(bin);
        }
    }

    binary
}

fn parse_line(line: &str) -> Option<u16> {
    let mut words = line.split_whitespace();

    let mut parameters = Vec::<Parameter>::new();

    if let Some(instruction_str) = words.next() {
        let instruction = parse_instruction(instruction_str);

        for parameter_str in words {
            parameters.push(parse_parameter(parameter_str));
        }

        Some(generate_instruction(&instruction, &parameters))
    } else {
        None
    }
}

fn parse_instruction(word: &str) -> Instruction {
    match word {
        "CLS" => Instruction::Cls,
        "RET" => Instruction::Ret,
        "JMP" => Instruction::Jmp,
        "CALL" => Instruction::Call,
        "SEQ" => Instruction::Seq,
        "SNE" => Instruction::Sne,
        "LD" => Instruction::Ld,
        "ADD" => Instruction::Add,
        "OR" => Instruction::Or,
        "AND" => Instruction::And,
        "XOR" => Instruction::Xor,
        "SUB" => Instruction::Sub,
        "SHR" => Instruction::Shr,
        "SUBN" => Instruction::Subn,
        "SHL" => Instruction::Shl,
        "JMPO" => Instruction::Jmpo,
        "RND" => Instruction::Rnd,
        "DRW" => Instruction::Drw,
        "SKP" => Instruction::Skp,
        "SKNP" => Instruction::Sknp,
        "LDK" => Instruction::Ldk,
        "SPR" => Instruction::Spr,
        "BCD" => Instruction::Bcd,
        "STN" => Instruction::Stn,
        "LDN" => Instruction::Ldn,
        _ => panic!("Unknown instruction name"),
    }
}

fn parse_parameter(word: &str) -> Parameter {
    if let Some(n_str) = word.strip_prefix('V') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 16 {
                Parameter::Register(Register::V(n as u8))
            } else {
                panic!("There are only 16 V register");
            }
        } else {
            panic!("Wrong V register name");
        }
    } else if word == "I" {
        Parameter::Register(Register::I)
    } else if word == "DT" {
        Parameter::Register(Register::DT)
    } else if word == "ST" {
        Parameter::Register(Register::ST)
    } else if let Some(n_str) = word.strip_suffix('A') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 4096 {
                Parameter::Address(n as u16)
            } else {
                panic!("Address can only take values up to 4095");
            }
        } else {
            panic!("Can't parse Address");
        }
    } else if let Some(n_str) = word.strip_suffix('B') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 256 {
                Parameter::Byte(n as u8)
            } else {
                panic!("Byte can only take values up to 255");
            }
        } else {
            panic!("Can't parse Byte");
        }
    } else if let Some(n_str) = word.strip_suffix('N') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 16 {
                Parameter::Nibble(n as u8)
            } else {
                panic!("Nibble can only take values up to 15");
            }
        } else {
            panic!("Can't parse Nibble");
        }
    } else {
        panic!("Unknown parameter type");
    }
}
