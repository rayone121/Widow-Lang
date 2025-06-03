use crate::vm::inst_type::InstructionType;

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
    }
}

pub fn decode(bits: u32) -> Result<InstructionType, String> {}
