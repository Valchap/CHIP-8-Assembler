use crate::instruction_generator::{generate_instruction, Instruction, Parameter, Register};

pub fn parse(text: &str) -> Result<Vec<u16>, String> {
    let mut binary = Vec::<u16>::new();

    let mut line_n = 1;

    for line in text.lines() {
        match parse_line(line) {
            Ok(line_binary) => {
                if let Some(bin) = line_binary {
                    binary.push(bin);
                }
            }
            Err(line_error) => return Err(format!("Error line {line_n} : {line_error}")),
        }

        line_n += 1;
    }

    Ok(binary)
}

fn parse_line(line: &str) -> Result<Option<u16>, String> {
    let mut words = line.split(';').next().unwrap().split_whitespace();

    let mut parameters = Vec::<Parameter>::new();

    if let Some(instruction_str) = words.next() {
        match parse_instruction(instruction_str) {
            Ok(instruction) => {
                for parameter_str in words {
                    match parse_parameter(parameter_str) {
                        Ok(parameter) => {
                            parameters.push(parameter);
                        }

                        Err(error) => return Err(error),
                    }
                }

                match generate_instruction(&instruction, &parameters) {
                    Ok(line_binary) => Ok(Some(line_binary)),

                    Err(line_error) => Err(line_error),
                }
            }

            Err(error) => Err(error),
        }
    } else {
        Ok(None)
    }
}

fn parse_instruction(word: &str) -> Result<Instruction, String> {
    match word {
        "CLS" => Ok(Instruction::Cls),
        "RET" => Ok(Instruction::Ret),
        "JMP" => Ok(Instruction::Jmp),
        "CALL" => Ok(Instruction::Call),
        "SEQ" => Ok(Instruction::Seq),
        "SNE" => Ok(Instruction::Sne),
        "LD" => Ok(Instruction::Ld),
        "ADD" => Ok(Instruction::Add),
        "OR" => Ok(Instruction::Or),
        "AND" => Ok(Instruction::And),
        "XOR" => Ok(Instruction::Xor),
        "SUB" => Ok(Instruction::Sub),
        "SHR" => Ok(Instruction::Shr),
        "SUBN" => Ok(Instruction::Subn),
        "SHL" => Ok(Instruction::Shl),
        "JMPO" => Ok(Instruction::Jmpo),
        "RND" => Ok(Instruction::Rnd),
        "DRW" => Ok(Instruction::Drw),
        "SKP" => Ok(Instruction::Skp),
        "SKNP" => Ok(Instruction::Sknp),
        "LDK" => Ok(Instruction::Ldk),
        "SPR" => Ok(Instruction::Spr),
        "BCD" => Ok(Instruction::Bcd),
        "STN" => Ok(Instruction::Stn),
        "LDN" => Ok(Instruction::Ldn),
        _ => Err("Unknown instruction name".to_owned()),
    }
}

fn parse_parameter(word: &str) -> Result<Parameter, String> {
    if let Some(n_str) = word.strip_prefix('V') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 16 {
                Ok(Parameter::Register(Register::V(n as u8)))
            } else {
                Err("There are only 16 V register".to_owned())
            }
        } else {
            Err("Wrong V register name".to_owned())
        }
    } else if word == "I" {
        Ok(Parameter::Register(Register::I))
    } else if word == "DT" {
        Ok(Parameter::Register(Register::DT))
    } else if word == "ST" {
        Ok(Parameter::Register(Register::ST))
    } else if let Some(n_str) = word.strip_suffix('A') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 4096 {
                Ok(Parameter::Address(n as u16))
            } else {
                Err("Address can only take values up to 4095".to_owned())
            }
        } else {
            Err("Can't parse Address".to_owned())
        }
    } else if let Some(n_str) = word.strip_suffix('B') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 256 {
                Ok(Parameter::Byte(n as u8))
            } else {
                Err("Byte can only take values up to 255".to_owned())
            }
        } else {
            Err("Can't parse Byte".to_owned())
        }
    } else if let Some(n_str) = word.strip_suffix('N') {
        if let Ok(n) = n_str.parse::<u64>() {
            if n < 16 {
                Ok(Parameter::Nibble(n as u8))
            } else {
                Err("Nibble can only take values up to 15".to_owned())
            }
        } else {
            Err("Can't parse Nibble".to_owned())
        }
    } else {
        Err("Unknown parameter type".to_owned())
    }
}
