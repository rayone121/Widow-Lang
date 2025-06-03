use crate::vm::opcode::Opcode;
use crate::vm::types::{FunctionCode, Register, ShiftAmount};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    RType {
        opcode: Opcode,
        rs: Register,
        rt: Register,
        rd: Register,
        shamt: ShiftAmount,
        funct: FunctionCode,
    },
    IType {
        opcode: Opcode,
        rs: Register,
        rt: Register,
        imm: u16,
    },
    JType {
        opcode: Opcode,
        target: u16,
    },
}
