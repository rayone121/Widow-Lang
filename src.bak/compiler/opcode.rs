#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    // Program control
    NOP = 0x00,  
    HALT = 0x01,

    // Data movement
    MOV = 0x02,

    // Arithmetic (register-register)
    ADD = 0x10,
    SUB = 0x11,
    MUL = 0x12,
    DIV = 0x13,

    // Arithmetic (register-immediate)
    ADDI = 0x14,

    // Logical (register-register)
    AND = 0x20,
    OR = 0x21,
    XOR = 0x22,

    // Logical (register-immediate)
    ANDI = 0x24,
    ORI = 0x25,
    XORI = 0x26,

    // Shifts
    SLL = 0x30,
    SRL = 0x31,

    // Comparison
    SLT = 0x40,

    // Branches
    BEQ = 0x50,
    BNE = 0x51,
    BLT = 0x52,
    BGE = 0x53,

    // Jumps
    JAL = 0x60,
    JALR = 0x61,

    // Memory
    LW = 0x70,
    SW = 0x71,

    // System
    SYSCALL = 0x80,
}
