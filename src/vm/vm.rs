use crate::compiler::{
    instruction_type::InstructionType,
    opcode::{RTypeOp, ITypeOp, BTypeOp, JTypeOp, MTypeOp, STypeOp, NTypeOp},
    register::Register,
    decode::decode,
};
use crate::vm::{
    error::{VMError, VMResult},
    memory::Memory,
    registers::RegisterFile,
    gc::{GarbageCollector, GCConfig},
};
use std::io::{self, Write};

/// The main virtual machine for executing bytecode
#[derive(Debug)]
pub struct VM {
    /// Register file (32 general-purpose registers)
    registers: RegisterFile,
    /// Memory subsystem
    memory: Memory,
    /// Garbage collector
    gc: GarbageCollector,
    /// Program counter
    pc: u32,
    /// Execution state
    running: bool,
    /// Instruction count for debugging/profiling
    instruction_count: u64,
    /// Automatic GC enabled
    auto_gc: bool,
}

impl VM {
    /// Create a new VM with specified memory size
    pub fn new(memory_size: u32) -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: Memory::new(memory_size),
            gc: GarbageCollector::new_default(),
            pc: 0,
            running: false,
            instruction_count: 0,
            auto_gc: true,
        }
    }

    /// Create a new VM with custom GC configuration
    pub fn new_with_gc(memory_size: u32, gc_config: GCConfig) -> Self {
        Self {
            registers: RegisterFile::new(),
            memory: Memory::new(memory_size),
            gc: GarbageCollector::new(gc_config),
            pc: 0,
            running: false,
            instruction_count: 0,
            auto_gc: true,
        }
    }

    /// Create a VM with default 16MB memory
    pub fn new_default() -> Self {
        Self::new(16 * 1024 * 1024) // 16MB
    }

    /// Load a program (bytecode) into memory
    pub fn load_program(&mut self, bytecode: &[u32]) -> VMResult<()> {
        self.memory.load_program(bytecode)?;
        self.pc = 0;
        self.running = false;
        self.instruction_count = 0;
        Ok(())
    }

    /// Run the program until halt or error
    pub fn run(&mut self) -> VMResult<()> {
        self.running = true;
        
        while self.running {
            self.step()?;
        }
        
        Ok(())
    }

    /// Execute a single instruction
    pub fn step(&mut self) -> VMResult<()> {
        if !self.running {
            return Err(VMError::ProgramHalted);
        }

        // Check if we should trigger garbage collection
        if self.auto_gc && self.gc.should_collect(&self.memory) {
            self.gc.collect(&mut self.memory, &self.registers)?;
        }

        // Fetch instruction
        let instruction_bits = self.memory.read_word(self.pc)?;
        let current_pc = self.pc; // Save current PC for branch calculations
        
        // Decode instruction
        let instruction = decode(instruction_bits)
            .map_err(|_| VMError::InvalidInstruction(instruction_bits))?;
        
        // Increment PC (most instructions advance by 4 bytes)
        self.pc += 4;
        self.instruction_count += 1;
        
        // Execute instruction
        self.execute_instruction(instruction, current_pc)?;
        
        Ok(())
    }

    /// Execute a decoded instruction
    fn execute_instruction(&mut self, instruction: InstructionType, current_pc: u32) -> VMResult<()> {
        match instruction {
            InstructionType::RType { opcode, rd, rs, rt } => {
                self.execute_rtype(opcode, rd, rs, rt)
            }
            InstructionType::IType { opcode, rd, rs, imm } => {
                self.execute_itype(opcode, rd, rs, imm)
            }
            InstructionType::BType { opcode, rs, rt, offset } => {
                self.execute_btype(opcode, rs, rt, offset, current_pc)
            }
            InstructionType::JType { opcode, addr } => {
                self.execute_jtype(opcode, addr)
            }
            InstructionType::MType { opcode, rd, rs, rt } => {
                self.execute_mtype(opcode, rd, rs, rt)
            }
            InstructionType::SType { opcode, rd, rs } => {
                self.execute_stype(opcode, rd, rs)
            }
            InstructionType::NType { opcode } => {
                self.execute_ntype(opcode)
            }
        }
    }

    /// Execute R-Type instructions
    fn execute_rtype(&mut self, opcode: RTypeOp, rd: Register, rs: Register, rt: Register) -> VMResult<()> {
        let rs_val = self.registers.read(rs.get_value())?;
        let rt_val = self.registers.read(rt.get_value())?;
        
        let result = match opcode {
            RTypeOp::ADD => rs_val.wrapping_add(rt_val),
            RTypeOp::SUB => rs_val.wrapping_sub(rt_val),
            RTypeOp::MUL => rs_val.wrapping_mul(rt_val),
            RTypeOp::DIV => {
                if rt_val == 0 {
                    return Err(VMError::DivisionByZero);
                }
                rs_val / rt_val
            }
            RTypeOp::MOV => rs_val,
            RTypeOp::AND => rs_val & rt_val,
            RTypeOp::OR => rs_val | rt_val,
            RTypeOp::XOR => rs_val ^ rt_val,
            RTypeOp::NOT => !rs_val,
        };
        
        self.registers.write(rd.get_value(), result)?;
        Ok(())
    }

    /// Execute I-Type instructions
    fn execute_itype(&mut self, opcode: ITypeOp, rd: Register, rs: Register, imm: u16) -> VMResult<()> {
        match opcode {
            ITypeOp::LI => {
                // Load immediate: rd = imm (sign-extended)
                let value = imm as i16 as i32;
                self.registers.write(rd.get_value(), value)?;
            }
            ITypeOp::ADDI => {
                // Add immediate: rd = rs + imm (sign-extended)
                let rs_val = self.registers.read(rs.get_value())?;
                let imm_val = imm as i16 as i32;
                let result = rs_val.wrapping_add(imm_val);
                self.registers.write(rd.get_value(), result)?;
            }
            ITypeOp::LOAD => {
                // Load: rd = memory[rs + offset]
                let rs_val = self.registers.read(rs.get_value())?;
                let address = (rs_val as u32).wrapping_add(imm as u32);
                let value = self.memory.read_word(address)?;
                self.registers.write(rd.get_value(), value as i32)?;
            }
            ITypeOp::STORE => {
                // Store: memory[rs + offset] = rd
                let rs_val = self.registers.read(rs.get_value())?;
                let rd_val = self.registers.read(rd.get_value())?;
                let address = (rs_val as u32).wrapping_add(imm as u32);
                self.memory.write_word(address, rd_val as u32)?;
            }
        }
        Ok(())
    }

    /// Execute B-Type instructions
    fn execute_btype(&mut self, opcode: BTypeOp, rs: Register, rt: Register, offset: u16, current_pc: u32) -> VMResult<()> {
        let rs_val = self.registers.read(rs.get_value())?;
        let rt_val = self.registers.read(rt.get_value())?;
        
        let should_branch = match opcode {
            BTypeOp::BEQ => rs_val == rt_val,
            BTypeOp::BNE => rs_val != rt_val,
            BTypeOp::BLT => rs_val < rt_val,
            BTypeOp::BGE => rs_val >= rt_val,
            BTypeOp::BZ => rs_val == 0,
            BTypeOp::BNZ => rs_val != 0,
        };
        
        if should_branch {
            // Calculate branch target relative to the next instruction (current_pc + 4)
            let base_addr = current_pc + 4;
            let offset_val = offset as i16 as i32;
            
            // Calculate target with proper bounds checking
            let target = if offset_val >= 0 {
                base_addr.saturating_add(offset_val as u32)
            } else {
                base_addr.saturating_sub((-offset_val) as u32)
            };
            
            // Validate branch target
            if target >= self.memory.get_stats().total_memory {
                return Err(VMError::InvalidJumpAddress(target));
            }
            
            self.pc = target;
        }
        
        Ok(())
    }

    /// Execute J-Type instructions
    fn execute_jtype(&mut self, opcode: JTypeOp, addr: u16) -> VMResult<()> {
        match opcode {
            JTypeOp::JMP => {
                // Jump to address
                let target = addr as u32;
                if target >= self.memory.get_stats().total_memory {
                    return Err(VMError::InvalidJumpAddress(target));
                }
                self.pc = target;
            }
            JTypeOp::CALL => {
                // Call function: push return address and jump
                let return_addr = self.pc;
                self.memory.stack_push(return_addr)?;
                
                let target = addr as u32;
                if target >= self.memory.get_stats().total_memory {
                    return Err(VMError::InvalidJumpAddress(target));
                }
                self.pc = target;
            }
            JTypeOp::RET => {
                // Return from function: pop return address
                let return_addr = self.memory.stack_pop()?;
                self.pc = return_addr;
            }
        }
        Ok(())
    }

    /// Execute M-Type instructions
    fn execute_mtype(&mut self, opcode: MTypeOp, rd: Register, rs: Register, rt: Register) -> VMResult<()> {
        match opcode {
            MTypeOp::ALLOC => {
                // Allocate memory: rd = allocate(rs bytes)
                let size = self.registers.read(rs.get_value())? as u32;
                let address = self.memory.allocate(size)?;
                
                // Register object with garbage collector
                self.gc.register_object(address, size);
                
                self.registers.write(rd.get_value(), address as i32)?;

                // Check if automatic GC should run
                if self.auto_gc && self.gc.should_collect(&self.memory) {
                    self.gc.collect(&mut self.memory, &self.registers)?;
                }
            }
            MTypeOp::FREE => {
                // Free memory: free(rs)
                let address = self.registers.read(rs.get_value())? as u32;
                
                // Unregister from garbage collector
                self.gc.unregister_object(address);
                
                self.memory.free(address)?;
            }
            MTypeOp::ALOAD => {
                // Array load: rd = array[rs + rt]
                let base = self.registers.read(rs.get_value())? as u32;
                let index = self.registers.read(rt.get_value())? as u32;
                let address = base.wrapping_add(index * 4); // Assuming 4-byte elements
                let value = self.memory.read_word(address)?;
                self.registers.write(rd.get_value(), value as i32)?;
            }
            MTypeOp::ASTORE => {
                // Array store: array[rs + rt] = rd
                let base = self.registers.read(rs.get_value())? as u32;
                let index = self.registers.read(rt.get_value())? as u32;
                let value = self.registers.read(rd.get_value())?;
                let address = base.wrapping_add(index * 4); // Assuming 4-byte elements
                self.memory.write_word(address, value as u32)?;
            }
        }
        Ok(())
    }

    /// Execute S-Type instructions
    fn execute_stype(&mut self, opcode: STypeOp, rd: Option<Register>, rs: Option<Register>) -> VMResult<()> {
        match opcode {
            STypeOp::PRINT => {
                // Print value from register
                if let Some(reg) = rs {
                    let value = self.registers.read(reg.get_value())?;
                    println!("{}", value);
                    io::stdout().flush().map_err(|e| VMError::IOError(e.to_string()))?;
                }
            }
            STypeOp::READ => {
                // Read integer from stdin
                if let Some(reg) = rd {
                    print!("Enter number: ");
                    io::stdout().flush().map_err(|e| VMError::IOError(e.to_string()))?;
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)
                        .map_err(|e| VMError::IOError(e.to_string()))?;
                    
                    let value: i32 = input.trim().parse()
                        .map_err(|e| VMError::IOError(format!("Invalid input: {}", e)))?;
                    
                    self.registers.write(reg.get_value(), value)?;
                }
            }
            STypeOp::SYSCALL => {
                // System call - simplified implementation
                let syscall_num = if let Some(reg) = rs {
                    self.registers.read(reg.get_value())?
                } else {
                    0
                };
                
                match syscall_num {
                    1 => {
                        // Exit syscall
                        self.running = false;
                    }
                    _ => {
                        return Err(VMError::SystemCallError(format!("Unknown syscall: {}", syscall_num)));
                    }
                }
            }
        }
        Ok(())
    }

    /// Execute N-Type instructions
    fn execute_ntype(&mut self, opcode: NTypeOp) -> VMResult<()> {
        match opcode {
            NTypeOp::NOP => {
                // No operation - do nothing
            }
            NTypeOp::HALT => {
                // Halt execution
                self.running = false;
            }
        }
        Ok(())
    }

    /// Reset the VM to initial state
    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.reset();
        self.gc = GarbageCollector::new(self.gc.get_config().clone());
        self.pc = 0;
        self.running = false;
        self.instruction_count = 0;
    }

    /// Get current program counter
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    /// Set program counter
    pub fn set_pc(&mut self, pc: u32) -> VMResult<()> {
        if pc >= self.memory.get_stats().total_memory {
            return Err(VMError::InvalidJumpAddress(pc));
        }
        self.pc = pc;
        Ok(())
    }

    /// Check if VM is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Get instruction count
    pub fn get_instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// Get register file reference
    pub fn get_registers(&self) -> &RegisterFile {
        &self.registers
    }

    /// Get memory reference
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    /// Get garbage collector reference
    pub fn get_gc(&self) -> &GarbageCollector {
        &self.gc
    }

    /// Get mutable garbage collector reference
    pub fn get_gc_mut(&mut self) -> &mut GarbageCollector {
        &mut self.gc
    }

    /// Enable or disable automatic garbage collection
    pub fn set_auto_gc(&mut self, enabled: bool) {
        self.auto_gc = enabled;
    }

    /// Force garbage collection
    pub fn force_gc(&mut self) -> VMResult<()> {
        self.gc.force_collect(&mut self.memory, &self.registers)
    }

    /// Perform minor garbage collection (young generation only)
    pub fn minor_gc(&mut self) -> VMResult<()> {
        self.gc.minor_collect(&mut self.memory, &self.registers)
    }

    /// Dump VM state for debugging
    pub fn dump_state(&self) -> String {
        let mut output = String::new();
        output.push_str("=== VM State ===\n");
        output.push_str(&format!("PC: 0x{:08X}\n", self.pc));
        output.push_str(&format!("Running: {}\n", self.running));
        output.push_str(&format!("Instructions executed: {}\n", self.instruction_count));
        output.push_str("\n");
        output.push_str(&self.registers.dump());
        output.push_str("\n");
        output.push_str(&format!("{}", self.memory.get_stats()));
        output.push_str("\n");
        output.push_str("=== Garbage Collector ===\n");
        output.push_str(&format!("Auto GC: {}\n", self.auto_gc));
        output.push_str(&format!("Objects tracked: {}\n", self.gc.object_count()));
        output.push_str(&format!("Collections: {}\n", self.gc.get_stats().collections_performed));
        output.push_str(&format!("Objects collected: {}\n", self.gc.get_stats().objects_collected));
        output.push_str(&format!("Bytes collected: {} bytes\n", self.gc.get_stats().bytes_collected));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::instruction_builder::{InstructionBuilder, registers::*};
    use crate::compiler::encode::encode;

    #[test]
    fn test_vm_creation() {
        let vm = VM::new_default();
        assert_eq!(vm.pc, 0);
        assert!(!vm.running);
        assert_eq!(vm.instruction_count, 0);
    }

    #[test]
    fn test_program_loading() {
        let mut vm = VM::new_default();
        let program = vec![0x12345678, 0xABCDEF00];
        
        assert!(vm.load_program(&program).is_ok());
        assert_eq!(vm.memory.read_word(0).unwrap(), 0x12345678);
        assert_eq!(vm.memory.read_word(4).unwrap(), 0xABCDEF00);
    }

    #[test]
    fn test_simple_execution() {
        let mut vm = VM::new_default();
        
        // Create a simple program: LI R1, 42; HALT
        let li_instr = InstructionBuilder::load_immediate(r1(), 42);
        let halt_instr = InstructionBuilder::halt();
        
        let program = vec![
            encode(li_instr),
            encode(halt_instr),
        ];
        
        vm.load_program(&program).unwrap();
        vm.run().unwrap();
        
        // Check that R1 contains 42
        assert_eq!(vm.registers.read(1).unwrap(), 42);
        assert!(!vm.running);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut vm = VM::new_default();
        
        // Program: LI R1, 10; LI R2, 5; ADD R3, R1, R2; HALT
        let program = vec![
            encode(InstructionBuilder::load_immediate(r1(), 10)),
            encode(InstructionBuilder::load_immediate(r2(), 5)),
            encode(InstructionBuilder::add(r3(), r1(), r2())),
            encode(InstructionBuilder::halt()),
        ];
        
        vm.load_program(&program).unwrap();
        vm.run().unwrap();
        
        assert_eq!(vm.registers.read(1).unwrap(), 10);
        assert_eq!(vm.registers.read(2).unwrap(), 5);
        assert_eq!(vm.registers.read(3).unwrap(), 15);
    }

    #[test]
    fn test_branch_instruction() {
        let mut vm = VM::new_default();
        
        // Simple test: load different values based on branch
        let program = vec![
            encode(InstructionBuilder::load_immediate(r1(), 5)),      // 0: LI R1, 5
            encode(InstructionBuilder::load_immediate(r2(), 10)),     // 4: LI R2, 10  
            encode(InstructionBuilder::branch_equal(r1(), r2(), 8)),  // 8: BEQ R1, R2, +8 (should not branch since 5 != 10)
            encode(InstructionBuilder::load_immediate(r3(), 42)),     // 12: LI R3, 42 (this should execute)
            encode(InstructionBuilder::halt()),                       // 16: HALT
            encode(InstructionBuilder::load_immediate(r3(), 99)),     // 20: LI R3, 99 (this should not execute)
            encode(InstructionBuilder::halt()),                       // 24: HALT
        ];
        
        vm.load_program(&program).unwrap();
        vm.run().unwrap();
        
        assert_eq!(vm.registers.read(1).unwrap(), 5);
        assert_eq!(vm.registers.read(2).unwrap(), 10);
        assert_eq!(vm.registers.read(3).unwrap(), 42); // Should be 42 because branch was not taken
    }

    #[test]
    fn test_division_by_zero() {
        let mut vm = VM::new_default();
        
        // Program: LI R1, 10; LI R2, 0; DIV R3, R1, R2; HALT
        let program = vec![
            encode(InstructionBuilder::load_immediate(r1(), 10)),
            encode(InstructionBuilder::load_immediate(r2(), 0)),
            encode(InstructionBuilder::div(r3(), r1(), r2())),
            encode(InstructionBuilder::halt()),
        ];
        
        vm.load_program(&program).unwrap();
        
        let result = vm.run();
        assert!(matches!(result, Err(VMError::DivisionByZero)));
    }
}