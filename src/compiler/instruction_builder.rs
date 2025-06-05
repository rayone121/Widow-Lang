use crate::compiler::{
    instruction_type::InstructionType,
    opcode::{RTypeOp, ITypeOp, BTypeOp, JTypeOp, MTypeOp, STypeOp, NTypeOp},
    register::Register,
};

/// Instruction builder for creating instructions with a fluent, ergonomic API
pub struct InstructionBuilder;

impl InstructionBuilder {
    // ===== R-Type Instructions (Register-Register operations) =====
    
    /// Create an ADD instruction: rd = rs + rt
    pub fn add(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::ADD,
            rd, rs, rt
        }
    }
    
    /// Create a SUB instruction: rd = rs - rt
    pub fn sub(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::SUB,
            rd, rs, rt
        }
    }
    
    /// Create a MUL instruction: rd = rs * rt
    pub fn mul(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::MUL,
            rd, rs, rt
        }
    }
    
    /// Create a DIV instruction: rd = rs / rt
    pub fn div(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::DIV,
            rd, rs, rt
        }
    }
    
    /// Create a MOV instruction: rd = rs (rt is ignored)
    pub fn mov(rd: Register, rs: Register) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::RType {
            opcode: RTypeOp::MOV,
            rd, rs, rt: zero_reg
        }
    }
    
    /// Create an AND instruction: rd = rs & rt
    pub fn and(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::AND,
            rd, rs, rt
        }
    }
    
    /// Create an OR instruction: rd = rs | rt
    pub fn or(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::OR,
            rd, rs, rt
        }
    }
    
    /// Create an XOR instruction: rd = rs ^ rt
    pub fn xor(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::RType {
            opcode: RTypeOp::XOR,
            rd, rs, rt
        }
    }
    
    /// Create a NOT instruction: rd = !rs (rt is ignored)
    pub fn not(rd: Register, rs: Register) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::RType {
            opcode: RTypeOp::NOT,
            rd, rs, rt: zero_reg
        }
    }
    
    // ===== I-Type Instructions (Immediate operations) =====
    
    /// Create a Load Immediate instruction: rd = imm
    pub fn load_immediate(rd: Register, imm: u16) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::IType {
            opcode: ITypeOp::LI,
            rd, rs: zero_reg, imm
        }
    }
    
    /// Create an Add Immediate instruction: rd = rs + imm
    pub fn add_immediate(rd: Register, rs: Register, imm: u16) -> InstructionType {
        InstructionType::IType {
            opcode: ITypeOp::ADDI,
            rd, rs, imm
        }
    }
    
    /// Create a LOAD instruction: rd = memory[rs + offset]
    pub fn load(rd: Register, rs: Register, offset: u16) -> InstructionType {
        InstructionType::IType {
            opcode: ITypeOp::LOAD,
            rd, rs, imm: offset
        }
    }
    
    /// Create a STORE instruction: memory[rs + offset] = rd
    pub fn store(rd: Register, rs: Register, offset: u16) -> InstructionType {
        InstructionType::IType {
            opcode: ITypeOp::STORE,
            rd, rs, imm: offset
        }
    }
    
    // ===== B-Type Instructions (Branch operations) =====
    
    /// Create a Branch if Equal instruction: if (rs == rt) jump to offset
    pub fn branch_equal(rs: Register, rt: Register, offset: u16) -> InstructionType {
        InstructionType::BType {
            opcode: BTypeOp::BEQ,
            rs, rt, offset
        }
    }
    
    /// Create a Branch if Not Equal instruction: if (rs != rt) jump to offset
    pub fn branch_not_equal(rs: Register, rt: Register, offset: u16) -> InstructionType {
        InstructionType::BType {
            opcode: BTypeOp::BNE,
            rs, rt, offset
        }
    }
    
    /// Create a Branch if Less Than instruction: if (rs < rt) jump to offset
    pub fn branch_less_than(rs: Register, rt: Register, offset: u16) -> InstructionType {
        InstructionType::BType {
            opcode: BTypeOp::BLT,
            rs, rt, offset
        }
    }
    
    /// Create a Branch if Greater or Equal instruction: if (rs >= rt) jump to offset
    pub fn branch_greater_equal(rs: Register, rt: Register, offset: u16) -> InstructionType {
        InstructionType::BType {
            opcode: BTypeOp::BGE,
            rs, rt, offset
        }
    }
    
    /// Create a Branch if Zero instruction: if (rs == 0) jump to offset
    pub fn branch_zero(rs: Register, offset: u16) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::BType {
            opcode: BTypeOp::BZ,
            rs, rt: zero_reg, offset
        }
    }
    
    /// Create a Branch if Not Zero instruction: if (rs != 0) jump to offset
    pub fn branch_not_zero(rs: Register, offset: u16) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::BType {
            opcode: BTypeOp::BNZ,
            rs, rt: zero_reg, offset
        }
    }
    
    // ===== J-Type Instructions (Jump operations) =====
    
    /// Create a Jump instruction: jump to addr
    pub fn jump(addr: u16) -> InstructionType {
        InstructionType::JType {
            opcode: JTypeOp::JMP,
            addr
        }
    }
    
    /// Create a Call instruction: call function at addr
    pub fn call(addr: u16) -> InstructionType {
        InstructionType::JType {
            opcode: JTypeOp::CALL,
            addr
        }
    }
    
    /// Create a Return instruction: return from function
    pub fn ret() -> InstructionType {
        InstructionType::JType {
            opcode: JTypeOp::RET,
            addr: 0 // Address is ignored for RET
        }
    }
    
    // ===== M-Type Instructions (Memory management) =====
    
    /// Create an Allocate instruction: rd = allocate(rs bytes)
    pub fn allocate(rd: Register, rs: Register) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::MType {
            opcode: MTypeOp::ALLOC,
            rd, rs, rt: zero_reg
        }
    }
    
    /// Create a Free instruction: free(rs)
    pub fn free(rs: Register) -> InstructionType {
        let zero_reg = Register::new(0).unwrap(); // Use register 0 as dummy
        InstructionType::MType {
            opcode: MTypeOp::FREE,
            rd: zero_reg, rs, rt: zero_reg
        }
    }
    
    /// Create an Array Load instruction: rd = array[rs + rt]
    pub fn array_load(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::MType {
            opcode: MTypeOp::ALOAD,
            rd, rs, rt
        }
    }
    
    /// Create an Array Store instruction: array[rs + rt] = rd
    pub fn array_store(rd: Register, rs: Register, rt: Register) -> InstructionType {
        InstructionType::MType {
            opcode: MTypeOp::ASTORE,
            rd, rs, rt
        }
    }
    
    // ===== S-Type Instructions (System/IO operations) =====
    
    /// Create a Print instruction: print(rs)
    pub fn print(rs: Register) -> InstructionType {
        InstructionType::SType {
            opcode: STypeOp::PRINT,
            rd: None,
            rs: Some(rs)
        }
    }
    
    /// Create a Read instruction: rd = read()
    pub fn read(rd: Register) -> InstructionType {
        InstructionType::SType {
            opcode: STypeOp::READ,
            rd: Some(rd),
            rs: None
        }
    }
    
    /// Create a System Call instruction
    pub fn syscall(rd: Option<Register>, rs: Option<Register>) -> InstructionType {
        InstructionType::SType {
            opcode: STypeOp::SYSCALL,
            rd, rs
        }
    }
    
    // ===== N-Type Instructions (No operand operations) =====
    
    /// Create a No Operation instruction
    pub fn nop() -> InstructionType {
        InstructionType::NType {
            opcode: NTypeOp::NOP
        }
    }
    
    /// Create a Halt instruction: stop execution
    pub fn halt() -> InstructionType {
        InstructionType::NType {
            opcode: NTypeOp::HALT
        }
    }
}

// ===== Convenience functions for common register creation =====

/// Helper functions for creating registers more easily
pub mod registers {
    use super::Register;
    
    /// Create register 0 (typically used as zero register)
    pub fn r0() -> Register {
        Register::new(0).unwrap()
    }
    
    /// Create register 1
    pub fn r1() -> Register {
        Register::new(1).unwrap()
    }
    
    /// Create register 2
    pub fn r2() -> Register {
        Register::new(2).unwrap()
    }
    
    /// Create register 3
    pub fn r3() -> Register {
        Register::new(3).unwrap()
    }
    
    /// Create register 4
    pub fn r4() -> Register {
        Register::new(4).unwrap()
    }
    
    /// Create register 5
    pub fn r5() -> Register {
        Register::new(5).unwrap()
    }
    
    /// Create register 6
    pub fn r6() -> Register {
        Register::new(6).unwrap()
    }
    
    /// Create register 7
    pub fn r7() -> Register {
        Register::new(7).unwrap()
    }
    
    /// Create register 8
    pub fn r8() -> Register {
        Register::new(8).unwrap()
    }
    
    /// Create register 9
    pub fn r9() -> Register {
        Register::new(9).unwrap()
    }
    
    /// Create register 10
    pub fn r10() -> Register {
        Register::new(10).unwrap()
    }
    
    /// Create register 11
    pub fn r11() -> Register {
        Register::new(11).unwrap()
    }
    
    /// Create register by number (0-31)
    pub fn reg(n: u8) -> Result<Register, String> {
        Register::new(n)
    }
    
    // Stack pointer convention (register 29)
    pub fn sp() -> Register {
        Register::new(29).unwrap()
    }
    
    // Frame pointer convention (register 30)
    pub fn fp() -> Register {
        Register::new(30).unwrap()
    }
    
    // Return address convention (register 31)
    pub fn ra() -> Register {
        Register::new(31).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::registers::*;
    
    #[test]
    fn test_r_type_instructions() {
        let add_instr = InstructionBuilder::add(r1(), r2(), r3());
        match add_instr {
            InstructionType::RType { opcode: RTypeOp::ADD, rd, rs, rt } => {
                assert_eq!(rd.get_value(), 1);
                assert_eq!(rs.get_value(), 2);
                assert_eq!(rt.get_value(), 3);
            }
            _ => panic!("Expected RType ADD instruction"),
        }
    }
    
    #[test]
    fn test_i_type_instructions() {
        let li_instr = InstructionBuilder::load_immediate(r1(), 42);
        match li_instr {
            InstructionType::IType { opcode: ITypeOp::LI, rd, imm, .. } => {
                assert_eq!(rd.get_value(), 1);
                assert_eq!(imm, 42);
            }
            _ => panic!("Expected IType LI instruction"),
        }
    }
    
    #[test]
    fn test_branch_instructions() {
        let beq_instr = InstructionBuilder::branch_equal(r1(), r2(), 100);
        match beq_instr {
            InstructionType::BType { opcode: BTypeOp::BEQ, rs, rt, offset } => {
                assert_eq!(rs.get_value(), 1);
                assert_eq!(rt.get_value(), 2);
                assert_eq!(offset, 100);
            }
            _ => panic!("Expected BType BEQ instruction"),
        }
    }
    
    #[test]
    fn test_jump_instructions() {
        let jump_instr = InstructionBuilder::jump(1000);
        match jump_instr {
            InstructionType::JType { opcode: JTypeOp::JMP, addr } => {
                assert_eq!(addr, 1000);
            }
            _ => panic!("Expected JType JMP instruction"),
        }
    }
    
    #[test]
    fn test_system_instructions() {
        let halt_instr = InstructionBuilder::halt();
        match halt_instr {
            InstructionType::NType { opcode: NTypeOp::HALT } => {
                // Success
            }
            _ => panic!("Expected NType HALT instruction"),
        }
    }
    
    #[test]
    fn test_convenience_registers() {
        assert_eq!(r0().get_value(), 0);
        assert_eq!(r1().get_value(), 1);
        assert_eq!(sp().get_value(), 29);
        assert_eq!(fp().get_value(), 30);
        assert_eq!(ra().get_value(), 31);
    }
}