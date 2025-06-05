use crate::compiler::{
    inst_type::InstructionType,
    opcode::Opcode,
    types::{FunctionCode, Register, ShiftAmount},
};

pub fn encode(instruction: InstructionType) -> u32 {
    match instruction {
        InstructionType::RType {
            opcode,
            rs,
            rt,
            rd,
            shamt,
            funct,
        } => {
            let opcode_bits = (opcode as u32) << 24;
            let rs_bits = (rs.get_value() as u32) << 20;
            let rt_bits = (rt.get_value() as u32) << 16;
            let rd_bits = (rd.get_value() as u32) << 12;
            let shamt_bits = (shamt.get_value() as u32) << 7;
            let funct_bits = funct.get_value() as u32;

            opcode_bits | rs_bits | rt_bits | rd_bits | shamt_bits | funct_bits
        }

        InstructionType::IType {
            opcode,
            rs,
            rt,
            imm,
        } => {
            let opcode_bits = (opcode as u32) << 24;
            let rs_bits = (rs.get_value() as u32) << 20;
            let rt_bits = (rt.get_value() as u32) << 16;
            let imm = imm as u32;

            opcode_bits | rs_bits | rt_bits | imm
        }
        InstructionType::JType { opcode, target } => {
            let opcode_bits = (opcode as u32) << 24;
            let target_bits = target as u32;

            opcode_bits | target_bits
        }
        InstructionType::NType { opcode } => {
            let opcode_bits = (opcode as u32) << 24;

            opcode_bits
        }
    }
}

pub fn decode(bits: u32) -> Result<InstructionType, String> {
    let opcode = u8_to_opcode((bits >> 24 & 0xFF) as u8)?;
    let instruction_type = opcode_to_instruction_type(&opcode);

    match instruction_type {
        "RType" => Ok({
            let rs = Register::new((bits >> 20 & 0xF) as u8)?;
            let rt = Register::new((bits >> 16 & 0xF) as u8)?;
            let rd = Register::new((bits >> 12 & 0xF) as u8)?;
            let shamt = ShiftAmount::new(((bits >> 7) & 0x1F) as u8)?;
            let funct = FunctionCode::new((bits & 0x7F) as u8)?;

            InstructionType::RType {
                opcode,
                rs,
                rt,
                rd,
                shamt,
                funct,
            }
        }),
        "IType" => Ok({
            let rs = Register::new((bits >> 20 & 0xF) as u8)?;
            let rt = Register::new((bits >> 16 & 0xF) as u8)?;
            let imm = (bits & 0xFFFF) as u16;

            InstructionType::IType {
                opcode,
                rs,
                rt,
                imm,
            }
        }),
        "JType" => Ok({
            let target = (bits & 0x3FFFFFF) as u16;

            InstructionType::JType { opcode, target }
        }),
        "NType" => Ok(InstructionType::NType { opcode }),
        _ => Err(format!("Unknown instruction type: {}", instruction_type)),
    }
}

fn u8_to_opcode(value: u8) -> Result<Opcode, String> {
    match value {
        0x00 => Ok(Opcode::NOP),
        0x01 => Ok(Opcode::HALT),
        0x02 => Ok(Opcode::MOV),
        0x10 => Ok(Opcode::ADD),
        0x11 => Ok(Opcode::SUB),
        0x12 => Ok(Opcode::MUL),
        0x13 => Ok(Opcode::DIV),
        0x14 => Ok(Opcode::ADDI),
        0x20 => Ok(Opcode::AND),
        0x21 => Ok(Opcode::OR),
        0x22 => Ok(Opcode::XOR),
        0x24 => Ok(Opcode::ANDI),
        0x25 => Ok(Opcode::ORI),
        0x26 => Ok(Opcode::XORI),
        0x30 => Ok(Opcode::SLL),
        0x31 => Ok(Opcode::SRL),
        0x40 => Ok(Opcode::SLT),
        0x50 => Ok(Opcode::BEQ),
        0x51 => Ok(Opcode::BNE),
        0x52 => Ok(Opcode::BLT),
        0x53 => Ok(Opcode::BGE),
        0x60 => Ok(Opcode::JAL),
        0x61 => Ok(Opcode::JALR),
        0x70 => Ok(Opcode::LW),
        0x71 => Ok(Opcode::SW),
        0x80 => Ok(Opcode::SYSCALL),
        _ => Err(format!("Unknown opcode: {:#04x}", value)),
    }
}

fn opcode_to_instruction_type(opcode: &Opcode) -> &str {
    match opcode {
        Opcode::NOP | Opcode::HALT => "NType",

        Opcode::MOV
        | Opcode::ADD
        | Opcode::SUB
        | Opcode::MUL
        | Opcode::DIV
        | Opcode::AND
        | Opcode::OR
        | Opcode::XOR
        | Opcode::SLL
        | Opcode::SRL
        | Opcode::SLT => "RType",

        Opcode::ADDI
        | Opcode::ANDI
        | Opcode::ORI
        | Opcode::XORI
        | Opcode::LW
        | Opcode::SW
        | Opcode::BEQ
        | Opcode::BNE
        | Opcode::BLT
        | Opcode::BGE => "IType",

        Opcode::JAL | Opcode::JALR => "JType",

        _ => "Unknown",
    }
}
