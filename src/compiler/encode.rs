use crate::compiler::instruction_type::InstructionType;

pub fn encode(instruction: InstructionType) -> u32 {
    match instruction {
        InstructionType::RType { opcode, rd, rs, rt } => {
            ((opcode as u8 as u32) << 24)
                | ((rd.get_value() as u32) << 19)
                | ((rs.get_value() as u32) << 14)
                | ((rt.get_value() as u32) << 9)
        }
        InstructionType::IType {
            opcode,
            rd,
            rs,
            imm,
        } => {
            ((opcode as u8 as u32) << 24)
                | ((rd.get_value() as u32) << 19)
                | ((rs.get_value() as u32) << 14)
                | (imm as u32)
        }
        InstructionType::BType {
            opcode,
            rs,
            rt,
            offset,
        } => {
            ((opcode as u8 as u32) << 24)
                | ((rs.get_value() as u32) << 19)
                | ((rt.get_value() as u32) << 14)
                | (offset as u32)
        }
        InstructionType::JType { opcode, addr } => ((opcode as u8 as u32) << 24) | (addr as u32),
        InstructionType::MType { opcode, rd, rs, rt } => {
            ((opcode as u8 as u32) << 24)
                | ((rd.get_value() as u32) << 19)
                | ((rs.get_value() as u32) << 14)
                | ((rt.get_value() as u32) << 9)
        }
        InstructionType::SType { opcode, rd, rs } => {
            ((opcode as u8 as u32) << 24)
                | (rd.map_or(0, |r| r.get_value() as u32) << 19)
                | (rs.map_or(0, |r| r.get_value() as u32) << 14)
        }
        InstructionType::NType { opcode } => (opcode as u8 as u32) << 24,
    }
}
