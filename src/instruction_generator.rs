pub enum Register {
    V(u8), // general purpose register
    I,     // address register
    ST,    // sound timer register
    DT,    // delay timer register
}

pub enum Instruction {
    Cls,
    Ret,
    Jmp,
    Call,
    Seq,
    Sne,
    Ld,
    Add,
    Or,
    And,
    Xor,
    Sub,
    Shr,
    Subn,
    Shl,
    Jmpo,
    Rnd,
    Drw,
    Skp,
    Sknp,
    Ldk,
    Spr,
    Bcd,
    Stn,
    Ldn,
}

pub enum Parameter {
    Register(Register), // register
    Address(u16),       // 12 bits value
    Byte(u8),           // 8 bits value
    Nibble(u8),         // 4 bits value
}

fn generate_cls(parameters: &[Parameter]) -> u16 {
    if !parameters.is_empty() {
        panic!("CLS takes no parameter");
    }

    0x00E0
}

fn generate_ret(parameters: &[Parameter]) -> u16 {
    if !parameters.is_empty() {
        panic!("RET takes no parameter");
    }

    0x00EE
}

fn generate_jmp(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("JMP takes one parameter");
    }

    if let Parameter::Address(nnn) = parameters[0] {
        0x1 << 12 | nnn
    } else {
        panic!("JMP first parameter must be Address");
    }
}

fn generate_call(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("CALL takes one parameter")
    }

    if let Parameter::Address(nnn) = parameters[0] {
        0x2 << 12 | nnn
    } else {
        panic!("CALL first parameter must be Address");
    }
}

fn generate_seq(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SEQ takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x5 << 12 | (x as u16) << 8 | (y as u16) << 4
        } else if let Parameter::Byte(nn) = parameters[1] {
            0x3 << 12 | (x as u16) << 8 | nn as u16
        } else {
            panic!("SEQ second parameter must be V[n] or byte")
        }
    } else {
        panic!("SEQ first parameter must be V[n]");
    }
}

fn generate_sne(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SNE takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x9 << 12 | (x as u16) << 8 | (y as u16) << 4
        } else if let Parameter::Byte(nn) = parameters[1] {
            0x4 << 12 | (x as u16) << 8 | nn as u16
        } else {
            panic!("SNE second parameter must be V[n] or byte")
        }
    } else {
        panic!("SNE first parameter must be V[n]");
    }
}

fn generate_ld(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("LD takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x5 << 12 | (x as u16) << 8 | (y as u16) << 4
        } else if let Parameter::Register(Register::DT) = parameters[1] {
            0xF << 12 | (x as u16) << 8 | 0x07
        } else if let Parameter::Byte(nn) = parameters[1] {
            0x6 << 12 | (x as u16) << 8 | nn as u16
        } else {
            panic!("LD V[n] second parameter must be V[n], DT or Byte");
        }
    } else if let Parameter::Register(Register::I) = parameters[0] {
        if let Parameter::Address(nnn) = parameters[1] {
            0xA << 12 | nnn
        } else {
            panic!("LD I second parameter must be Address")
        }
    } else if let Parameter::Register(Register::DT) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            0xF << 12 | (x as u16) << 8 | 0x15
        } else {
            panic!("LD DT second parameter must be V[n]");
        }
    } else if let Parameter::Register(Register::ST) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            0xF << 12 | (x as u16) << 8 | 0x18
        } else {
            panic!("LD ST second parameter must be V[n]");
        }
    } else {
        panic!("LD first parameter must be V[n], I, DT or ST");
    }
}

fn generate_add(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("ADD takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x4
        } else if let Parameter::Byte(nn) = parameters[1] {
            0x7 << 12 | (x as u16) << 8 | nn as u16
        } else {
            panic!("ADD V[n] second parameter must be V[n] or Byte");
        }
    } else if let Parameter::Register(Register::I) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            0xF << 12 | (x as u16) << 8 | 0x1E
        } else {
            panic!("ADD I second parameter must be V[n]");
        }
    } else {
        panic!("ADD first parameter must be V[n] or I");
    }
}

fn generate_or(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("OR takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x1
        } else {
            panic!("OR second parameter must be V[n]")
        }
    } else {
        panic!("OR first parameter must be V[n]");
    }
}

fn generate_and(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("AND takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x2
        } else {
            panic!("AND second parameter must be V[n]")
        }
    } else {
        panic!("AND first parameter must be V[n]");
    }
}

fn generate_xor(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("XOR takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x3
        } else {
            panic!("XOR second parameter must be V[n]")
        }
    } else {
        panic!("XOR first parameter must be V[n]");
    }
}

fn generate_sub(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SUB takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x5
        } else {
            panic!("SUB second parameter must be V[n]")
        }
    } else {
        panic!("SUB first parameter must be V[n]");
    }
}

fn generate_shr(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SHR takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x6
        } else {
            panic!("SHR second parameter must be V[n]")
        }
    } else {
        panic!("SHR first parameter must be V[n]");
    }
}

fn generate_subn(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SUBN takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x7
        } else {
            panic!("SUBN second parameter must be V[n]")
        }
    } else {
        panic!("SUBN first parameter must be V[n]");
    }
}

fn generate_shl(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("SHL takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0xE
        } else {
            panic!("SHL second parameter must be V[n]")
        }
    } else {
        panic!("SHL first parameter must be V[n]");
    }
}

fn generate_jmpo(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("JMPO takes one parameter");
    }

    if let Parameter::Address(nnn) = parameters[0] {
        0xB << 12 | nnn
    } else {
        panic!("JMPO first parameter must be an Address");
    }
}

fn generate_rnd(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 2 {
        panic!("RND takes two parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Byte(nn) = parameters[1] {
            0xC << 12 | (x as u16) << 8 | nn as u16
        } else {
            panic!("RND second parameter must be a Byte");
        }
    } else {
        panic!("RND first parameter must be V[n]");
    }
}

fn generate_drw(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 3 {
        panic!("DRW takes three parameters");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            if let Parameter::Nibble(n) = parameters[2] {
                0xD << 12 | (x as u16) << 8 | (y as u16) << 4 | n as u16
            } else {
                panic!("DRW third parameter must be a Nibble");
            }
        } else {
            panic!("DRW second parameter must be V[n]");
        }
    } else {
        panic!("DRW first parameter must be V[n]");
    }
}

fn generate_skp(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("SKP takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xE << 12 | (x as u16) << 8 | 0x9E
    } else {
        panic!("SKP first parameter must be V[n]");
    }
}

fn generate_sknp(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("SKNP takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xE << 12 | (x as u16) << 8 | 0xA1
    } else {
        panic!("SKNP first parameter must be V[n]");
    }
}

fn generate_ldk(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("LDK takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xF << 12 | (x as u16) << 8 | 0x15
    } else {
        panic!("LDK first parameter must be V[n]");
    }
}

fn generate_spr(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("SPR takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xF << 12 | (x as u16) | 0x29
    } else {
        panic!("SPR first parameter must be V[n]");
    }
}

fn generate_bcd(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("BCD takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xF << 12 | (x as u16) | 0x33
    } else {
        panic!("BCD first parameter must be V[n]");
    }
}

fn generate_stn(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("STN takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xF << 12 | (x as u16) | 0x55
    } else {
        panic!("STN first parameter must be V[n]");
    }
}

fn generate_ldn(parameters: &[Parameter]) -> u16 {
    if parameters.len() != 1 {
        panic!("LDN takes one parameter");
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        0xF << 12 | (x as u16) | 0x65
    } else {
        panic!("LDN first parameter must be V[n]");
    }
}

pub fn generate_instruction(instruction: Instruction, parameters: &[Parameter]) -> u16 {
    match instruction {
        Instruction::Cls => generate_cls(parameters),
        Instruction::Ret => generate_ret(parameters),
        Instruction::Jmp => generate_jmp(parameters),
        Instruction::Call => generate_call(parameters),
        Instruction::Seq => generate_seq(parameters),
        Instruction::Sne => generate_sne(parameters),
        Instruction::Ld => generate_ld(parameters),
        Instruction::Add => generate_add(parameters),
        Instruction::Or => generate_or(parameters),
        Instruction::And => generate_and(parameters),
        Instruction::Xor => generate_xor(parameters),
        Instruction::Sub => generate_sub(parameters),
        Instruction::Shr => generate_shr(parameters),
        Instruction::Subn => generate_subn(parameters),
        Instruction::Shl => generate_shl(parameters),
        Instruction::Jmpo => generate_jmpo(parameters),
        Instruction::Rnd => generate_rnd(parameters),
        Instruction::Drw => generate_drw(parameters),
        Instruction::Skp => generate_skp(parameters),
        Instruction::Sknp => generate_sknp(parameters),
        Instruction::Ldk => generate_ldk(parameters),
        Instruction::Spr => generate_spr(parameters),
        Instruction::Bcd => generate_bcd(parameters),
        Instruction::Stn => generate_stn(parameters),
        Instruction::Ldn => generate_ldn(parameters),
    }
}
