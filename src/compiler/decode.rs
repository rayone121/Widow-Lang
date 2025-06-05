use crate::compiler::{
    instruction_type::InstructionType,
    opcode::{BTypeOp, ITypeOp, JTypeOp, MTypeOp, NTypeOp, RTypeOp, STypeOp},
    register::Register,
};

pub fn decode(bits: u32) -> Result<InstructionType, String> {
    let opcode_byte = ((bits >> 24) & 0xFF) as u8;

    match opcode_byte {
        // R-Type instructions
        0x10..=0x14 | 0x20..=0x23 => decode_rtype(bits, opcode_byte),
        // I-Type instructions
        0x30..=0x31 | 0x40..=0x41 => decode_itype(bits, opcode_byte),
        // B-Type instructions
        0x50..=0x55 => decode_btype(bits, opcode_byte),
        // J-Type instructions
        0x60..=0x62 => decode_jtype(bits, opcode_byte),
        // M-Type instructions
        0x70..=0x73 => decode_mtype(bits, opcode_byte),
        // S-Type instructions
        0x80..=0x82 => decode_stype(bits, opcode_byte),
        // N-Type instructions
        0x00..=0x01 => decode_ntype(bits, opcode_byte),
        _ => Err(format!("Invalid opcode: 0x{:02X}", opcode_byte)),
    }
}

fn decode_rtype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x10 => RTypeOp::ADD,
        0x11 => RTypeOp::SUB,
        0x12 => RTypeOp::MUL,
        0x13 => RTypeOp::DIV,
        0x14 => RTypeOp::MOV,
        0x20 => RTypeOp::AND,
        0x21 => RTypeOp::OR,
        0x22 => RTypeOp::XOR,
        0x23 => RTypeOp::NOT,
        _ => unreachable!(), // Already validated by range
    };

    let rd = Register::new(((bits >> 19) & 0x1F) as u8)?;
    let rs = Register::new(((bits >> 14) & 0x1F) as u8)?;
    let rt = Register::new(((bits >> 9) & 0x1F) as u8)?;

    Ok(InstructionType::RType { opcode, rd, rs, rt })
}

fn decode_itype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x30 => ITypeOp::LI,
        0x31 => ITypeOp::ADDI,
        0x40 => ITypeOp::LOAD,
        0x41 => ITypeOp::STORE,
        _ => unreachable!(), // Already validated by range
    };

    let rd = Register::new(((bits >> 19) & 0x1F) as u8)?;
    let rs = Register::new(((bits >> 14) & 0x1F) as u8)?;
    let imm = (bits & 0xFFFF) as u16;

    Ok(InstructionType::IType {
        opcode,
        rd,
        rs,
        imm,
    })
}

fn decode_btype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x50 => BTypeOp::BEQ,
        0x51 => BTypeOp::BNE,
        0x52 => BTypeOp::BLT,
        0x53 => BTypeOp::BGE,
        0x54 => BTypeOp::BZ,
        0x55 => BTypeOp::BNZ,
        _ => unreachable!(), // Already validated by range
    };

    let rs = Register::new(((bits >> 19) & 0x1F) as u8)?;
    let rt = Register::new(((bits >> 14) & 0x1F) as u8)?;
    let offset = (bits & 0xFFFF) as u16;

    Ok(InstructionType::BType {
        opcode,
        rs,
        rt,
        offset,
    })
}

fn decode_jtype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x60 => JTypeOp::JMP,
        0x61 => JTypeOp::CALL,
        0x62 => JTypeOp::RET,
        _ => unreachable!(), // Already validated by range
    };

    let addr = (bits & 0xFFFF) as u16;

    Ok(InstructionType::JType { opcode, addr })
}

fn decode_mtype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x70 => MTypeOp::ALLOC,
        0x71 => MTypeOp::FREE,
        0x72 => MTypeOp::ALOAD,
        0x73 => MTypeOp::ASTORE,
        _ => unreachable!(), // Already validated by range
    };

    let rd = Register::new(((bits >> 19) & 0x1F) as u8)?;
    let rs = Register::new(((bits >> 14) & 0x1F) as u8)?;
    let rt = Register::new(((bits >> 9) & 0x1F) as u8)?;

    Ok(InstructionType::MType { opcode, rd, rs, rt })
}

fn decode_stype(bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x80 => STypeOp::PRINT,
        0x81 => STypeOp::READ,
        0x82 => STypeOp::SYSCALL,
        _ => unreachable!(), // Already validated by range
    };

    let rd = Some(Register::new(((bits >> 19) & 0x1F) as u8)?);
    let rs = Some(Register::new(((bits >> 14) & 0x1F) as u8)?);

    Ok(InstructionType::SType { opcode, rd, rs })
}

fn decode_ntype(_bits: u32, opcode_byte: u8) -> Result<InstructionType, String> {
    let opcode = match opcode_byte {
        0x00 => NTypeOp::NOP,
        0x01 => NTypeOp::HALT,
        _ => unreachable!(), // Already validated by range
    };

    Ok(InstructionType::NType { opcode })
}
