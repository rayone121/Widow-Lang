#[derive(Debug, Clone, PartialEq)]
pub enum VMError {
    // Memory related errors
    InvalidMemoryAddress(u32),
    MemoryAccessViolation(u32),
    OutOfMemory,
    
    // Register related errors
    InvalidRegister(u8),
    
    // Execution errors
    DivisionByZero,
    InvalidInstruction(u32),
    StackOverflow,
    StackUnderflow,
    
    // Jump/Branch errors
    InvalidJumpAddress(u32),
    InvalidBranchOffset(i16),
    
    // System errors
    IOError(String),
    SystemCallError(String),
    
    // Runtime errors
    ProgramHalted,
    InvalidOpcode(u8),
    
    // Memory allocation errors
    AllocationFailed(u32), // Failed to allocate N bytes
    FreeFailed(u32),       // Failed to free address
    DoubleFree(u32),       // Attempted to free already freed memory
    UseAfterFree(u32),     // Attempted to use freed memory
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VMError::InvalidMemoryAddress(addr) => {
                write!(f, "Invalid memory address: 0x{:08X}", addr)
            }
            VMError::MemoryAccessViolation(addr) => {
                write!(f, "Memory access violation at address: 0x{:08X}", addr)
            }
            VMError::OutOfMemory => write!(f, "Out of memory"),
            VMError::InvalidRegister(reg) => write!(f, "Invalid register: R{}", reg),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::InvalidInstruction(bits) => {
                write!(f, "Invalid instruction: 0x{:08X}", bits)
            }
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::InvalidJumpAddress(addr) => {
                write!(f, "Invalid jump address: 0x{:08X}", addr)
            }
            VMError::InvalidBranchOffset(offset) => {
                write!(f, "Invalid branch offset: {}", offset)
            }
            VMError::IOError(msg) => write!(f, "I/O error: {}", msg),
            VMError::SystemCallError(msg) => write!(f, "System call error: {}", msg),
            VMError::ProgramHalted => write!(f, "Program execution halted"),
            VMError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: 0x{:02X}", opcode),
            VMError::AllocationFailed(size) => {
                write!(f, "Memory allocation failed for {} bytes", size)
            }
            VMError::FreeFailed(addr) => {
                write!(f, "Failed to free memory at address: 0x{:08X}", addr)
            }
            VMError::DoubleFree(addr) => {
                write!(f, "Double free detected at address: 0x{:08X}", addr)
            }
            VMError::UseAfterFree(addr) => {
                write!(f, "Use after free detected at address: 0x{:08X}", addr)
            }
        }
    }
}

impl std::error::Error for VMError {}

pub type VMResult<T> = Result<T, VMError>;