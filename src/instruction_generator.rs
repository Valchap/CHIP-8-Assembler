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

fn generate_cls(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("CLS takes no parameter".to_owned());
    }
    Ok(0x00E0)
}

fn generate_ret(parameters: &[Parameter]) -> Result<u16, String> {
    if !parameters.is_empty() {
        return Err("RET takes no parameter".to_owned());
    }
    Ok(0x00EE)
}

fn generate_jmp(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("JMP takes one parameter".to_owned());
    }

    if let Parameter::Address(nnn) = parameters[0] {
        return Ok(0x1 << 12 | nnn);
    }
    Err("JMP first parameter must be Address".to_owned())
}

fn generate_call(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("CALL takes one parameter".to_owned());
    }

    if let Parameter::Address(nnn) = parameters[0] {
        return Ok(0x2 << 12 | nnn);
    }
    Err("CALL first parameter must be Address".to_owned())
}

fn generate_seq(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SEQ takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x5 << 12 | (x as u16) << 8 | (y as u16) << 4);
        } else if let Parameter::Byte(nn) = parameters[1] {
            return Ok(0x3 << 12 | (x as u16) << 8 | nn as u16);
        }
        return Err("SEQ second parameter must be V[n] or byte".to_owned());
    }
    Err("SEQ first parameter must be V[n]".to_owned())
}

fn generate_sne(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SNE takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x9 << 12 | (x as u16) << 8 | (y as u16) << 4);
        } else if let Parameter::Byte(nn) = parameters[1] {
            return Ok(0x4 << 12 | (x as u16) << 8 | nn as u16);
        }
        return Err("SNE second parameter must be V[n] or byte".to_owned());
    }
    Err("SNE first parameter must be V[n]".to_owned())
}

fn generate_ld(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("LD takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4);
        } else if let Parameter::Register(Register::DT) = parameters[1] {
            return Ok(0xF << 12 | (x as u16) << 8 | 0x07);
        } else if let Parameter::Byte(nn) = parameters[1] {
            return Ok(0x6 << 12 | (x as u16) << 8 | nn as u16);
        }

        return Err("LD V[n] second parameter must be V[n], DT or Byte".to_owned());
    } else if let Parameter::Register(Register::I) = parameters[0] {
        if let Parameter::Address(nnn) = parameters[1] {
            return Ok(0xA << 12 | nnn);
        }
        return Err("LD I second parameter must be Address".to_owned());
    } else if let Parameter::Register(Register::DT) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            return Ok(0xF << 12 | (x as u16) << 8 | 0x15);
        }
        return Err("LD DT second parameter must be V[n]".to_owned());
    } else if let Parameter::Register(Register::ST) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            return Ok(0xF << 12 | (x as u16) << 8 | 0x18);
        }
        return Err("LD ST second parameter must be V[n]".to_owned());
    }
    Err("LD first parameter must be V[n], I, DT or ST".to_owned())
}

fn generate_add(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("ADD takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x4);
        } else if let Parameter::Byte(nn) = parameters[1] {
            return Ok(0x7 << 12 | (x as u16) << 8 | nn as u16);
        }
        return Err("ADD V[n] second parameter must be V[n] or Byte".to_owned());
    } else if let Parameter::Register(Register::I) = parameters[0] {
        if let Parameter::Register(Register::V(x)) = parameters[1] {
            return Ok(0xF << 12 | (x as u16) << 8 | 0x1E);
        }
        return Err("ADD I second parameter must be V[n]".to_owned());
    }
    Err("ADD first parameter must be V[n] or I".to_owned())
}

fn generate_or(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("OR takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x1);
        }
        return Err("OR second parameter must be V[n]".to_owned());
    }
    Err("OR first parameter must be V[n]".to_owned())
}

fn generate_and(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("AND takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x2);
        }
        return Err("AND second parameter must be V[n]".to_owned());
    }
    Err("AND first parameter must be V[n]".to_owned())
}

fn generate_xor(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("XOR takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x3);
        }
        return Err("XOR second parameter must be V[n]".to_string());
    }
    Err("XOR first parameter must be V[n]".to_string())
}

fn generate_sub(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SUB takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x5);
        }
        return Err("SUB second parameter must be V[n]".to_owned());
    }
    Err("SUB first parameter must be V[n]".to_owned())
}

fn generate_shr(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SHR takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x6);
        }
        return Err("SHR second parameter must be V[n]".to_owned());
    }
    Err("SHR first parameter must be V[n]".to_owned())
}

fn generate_subn(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SUBN takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0x7);
        }
        return Err("SUBN second parameter must be V[n]".to_owned());
    }
    Err("SUBN first parameter must be V[n]".to_owned())
}

fn generate_shl(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("SHL takes two parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            return Ok(0x8 << 12 | (x as u16) << 8 | (y as u16) << 4 | 0xE);
        }
        return Err("SHL second parameter must be V[n]".to_owned());
    }
    Err("SHL first parameter must be V[n]".to_owned())
}

fn generate_jmpo(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("JMPO takes one parameter".to_owned());
    }

    if let Parameter::Address(nnn) = parameters[0] {
        return Ok(0xB << 12 | nnn);
    }
    Err("JMPO first parameter must be an Address".to_string())
}

fn generate_rnd(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 2 {
        return Err("RND takes two parameters".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Byte(nn) = parameters[1] {
            return Ok(0xC << 12 | (x as u16) << 8 | nn as u16);
        }
        return Err("RND second parameter must be a Byte".to_string());
    }
    Err("RND first parameter must be V[n]".to_string())
}

fn generate_drw(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 3 {
        return Err("DRW takes three parameters".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        if let Parameter::Register(Register::V(y)) = parameters[1] {
            if let Parameter::Nibble(n) = parameters[2] {
                return Ok(0xD << 12 | (x as u16) << 8 | (y as u16) << 4 | n as u16);
            }
            return Err("DRW third parameter must be a Nibble".to_string());
        }
        return Err("DRW second parameter must be V[n]".to_string());
    }
    Err("DRW first parameter must be V[n]".to_string())
}

fn generate_skp(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("SKP takes one parameter".to_owned());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xE << 12 | (x as u16) << 8 | 0x9E);
    }
    Err("SKP first parameter must be V[n]".to_owned())
}

fn generate_sknp(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("SKNP takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xE << 12 | (x as u16) << 8 | 0xA1);
    }
    Err("SKNP first parameter must be V[n]".to_string())
}

fn generate_ldk(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("LDK takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xF << 12 | (x as u16) << 8 | 0x0A);
    }
    Err("LDK first parameter must be V[n]".to_string())
}

fn generate_spr(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("SPR takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xF << 12 | (x as u16) << 8 | 0x29);
    }
    Err("SPR first parameter must be V[n]".to_string())
}

fn generate_bcd(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("BCD takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xF << 12 | (x as u16) << 8 | 0x33);
    }
    Err("BCD first parameter must be V[n]".to_string())
}

fn generate_stn(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("STN takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xF << 12 | (x as u16) << 8 | 0x55);
    }
    Err("STN first parameter must be V[n]".to_string())
}

fn generate_ldn(parameters: &[Parameter]) -> Result<u16, String> {
    if parameters.len() != 1 {
        return Err("LDN takes one parameter".to_string());
    }

    if let Parameter::Register(Register::V(x)) = parameters[0] {
        return Ok(0xF << 12 | (x as u16) << 8 | 0x65);
    }
    Err("LDN first parameter must be V[n]".to_string())
}

pub fn generate_instruction(
    instruction: &Instruction,
    parameters: &[Parameter],
) -> Result<u16, String> {
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
