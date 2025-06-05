use crate::compiler::{
    opcode::{BTypeOp, ITypeOp, JTypeOp, MTypeOp, NTypeOp, RTypeOp, STypeOp},
    register::Register,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    RType {
        opcode: RTypeOp,
        rd: Register,
        rs: Register,
        rt: Register,
    },
    IType {
        opcode: ITypeOp,
        rd: Register,
        rs: Register,
        imm: u16,
    },
    BType {
        opcode: BTypeOp,
        rs: Register,
        rt: Register,
        offset: u16,
    },
    JType {
        opcode: JTypeOp,
        addr: u16,
    },
    MType {
        opcode: MTypeOp,
        rd: Register,
        rs: Register,
        rt: Register,
    },
    SType {
        opcode: STypeOp,
        rd: Option<Register>, // For READ
        rs: Option<Register>, // For PRINT
    },
    NType {
        opcode: NTypeOp,
    },
}
