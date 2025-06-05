#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RTypeOp {
    //Core Arithmetic
    ADD = 0x10, // rd = rs1 + rs2
    SUB = 0x11, // rd = rs1 - rs2
    MUL = 0x12, // rd = rs1 * rs2
    DIV = 0x13, // rd = rs1 / rs2
    MOV = 0x14, // rd = rs1

    //Core Logical
    AND = 0x20, // rd = rs1 & rs2
    OR = 0x21,  // rd = rs1 | rs2
    XOR = 0x22, // rd = rs1 ^ rs2
    NOT = 0x23, // rd = !rs1
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ITypeOp {
    //Load Constants
    LI = 0x30,   // rd = immediate (load immediate)
    ADDI = 0x31, // rd = rs + immediate

    //Memory
    LOAD = 0x40,  // rd = memory[rs + offset]
    STORE = 0x41, // memory[rs + offset] = rt
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BTypeOp {
    //Comparisons & Branches
    BEQ = 0x50, // if (rs1 == rs2) jump to offset
    BNE = 0x51, // if (rs1 != rs2) jump to offset
    BLT = 0x52, // if (rs1 < rs2) jump to offset
    BGE = 0x53, // if (rs1 >= rs2) jump to offset
    BZ = 0x54,  // if (rs == 0) jump to offset
    BNZ = 0x55, // if (rs != 0) jump to offset
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JTypeOp {
    //Control Flow
    JMP = 0x60,  // Jump to address
    CALL = 0x61, // Call function at address
    RET = 0x62,  // Return from function
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MTypeOp {
    //Dynamic Memory
    ALLOC = 0x70,  // rd = allocate(rs bytes)
    FREE = 0x71,   // free(rs)
    ALOAD = 0x72,  // rd = array[rs1 + rs2]
    ASTORE = 0x73, // array[rs1 + rs2] = rt
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum STypeOp {
    //I/O & System
    PRINT = 0x80,   // print(rs)
    READ = 0x81,    // rd = READ()
    SYSCALL = 0x82, // System call
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NTypeOp {
    NOP = 0x00,  // No operation
    HALT = 0x01, // Stop execution
}
