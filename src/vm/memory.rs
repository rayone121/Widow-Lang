use crate::vm::error::{VMError, VMResult};
use std::collections::HashMap;

/// Memory subsystem for the VM with heap and stack management
#[derive(Debug, Clone)]
pub struct Memory {
    /// Main memory storage
    memory: Vec<u8>,
    /// Current stack pointer
    stack_pointer: u32,
    /// Stack base (grows downward from here)
    stack_base: u32,
    /// Heap pointer (grows upward from here)
    heap_pointer: u32,
    /// Heap base
    heap_base: u32,
    /// Allocated blocks tracking for heap management
    allocated_blocks: HashMap<u32, u32>, // address -> size
    /// Memory size in bytes
    memory_size: u32,
}

impl Memory {
    /// Create a new memory subsystem
    /// 
    /// Memory layout:
    /// ```text
    /// 0x00000000 - 0x00010000: Code section (64KB)
    /// 0x00010000 - heap_top:   Heap (grows upward)
    /// stack_base - 0xFFFFFFFF: Stack (grows downward)
    /// ```
    pub fn new(memory_size: u32) -> Self {
        let code_section_size = if memory_size > 0x10000 { 0x10000 } else { memory_size / 4 }; // 64KB for code or 1/4 of total
        let stack_size = if memory_size > 0x100000 { 0x100000 } else { memory_size / 4 }; // 1MB for stack or 1/4 of total
        
        let heap_base = code_section_size;
        let stack_base = if memory_size > stack_size { memory_size - stack_size } else { memory_size * 3 / 4 };
        
        Self {
            memory: vec![0; memory_size as usize],
            stack_pointer: stack_base,
            stack_base,
            heap_pointer: heap_base,
            heap_base,
            allocated_blocks: HashMap::new(),
            memory_size,
        }
    }

    /// Read a byte from memory
    pub fn read_byte(&self, address: u32) -> VMResult<u8> {
        if address >= self.memory_size {
            return Err(VMError::InvalidMemoryAddress(address));
        }
        Ok(self.memory[address as usize])
    }

    /// Write a byte to memory
    pub fn write_byte(&mut self, address: u32, value: u8) -> VMResult<()> {
        if address >= self.memory_size {
            return Err(VMError::InvalidMemoryAddress(address));
        }
        
        // Check if writing to code section (might want to prevent this)
        if address < 0x10000 {
            // For now, allow writes to code section (for loading programs)
            // Could add a protection flag later
        }
        
        self.memory[address as usize] = value;
        Ok(())
    }

    /// Read a 32-bit word from memory (little-endian)
    pub fn read_word(&self, address: u32) -> VMResult<u32> {
        if address + 3 >= self.memory_size {
            return Err(VMError::InvalidMemoryAddress(address));
        }
        
        let bytes = [
            self.memory[address as usize],
            self.memory[(address + 1) as usize],
            self.memory[(address + 2) as usize],
            self.memory[(address + 3) as usize],
        ];
        
        Ok(u32::from_le_bytes(bytes))
    }

    /// Write a 32-bit word to memory (little-endian)
    pub fn write_word(&mut self, address: u32, value: u32) -> VMResult<()> {
        if address + 3 >= self.memory_size {
            return Err(VMError::InvalidMemoryAddress(address));
        }
        
        let bytes = value.to_le_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            self.memory[(address + i as u32) as usize] = byte;
        }
        
        Ok(())
    }

    /// Load bytecode into the code section
    pub fn load_program(&mut self, bytecode: &[u32]) -> VMResult<()> {
        let required_size = bytecode.len() * 4;
        if required_size > 0x10000 {
            return Err(VMError::OutOfMemory);
        }
        
        for (i, &instruction) in bytecode.iter().enumerate() {
            let address = (i * 4) as u32;
            self.write_word(address, instruction)?;
        }
        
        Ok(())
    }

    /// Allocate memory on the heap
    pub fn allocate(&mut self, size: u32) -> VMResult<u32> {
        if size == 0 {
            return Err(VMError::AllocationFailed(size));
        }
        
        // Align to 4-byte boundary
        let aligned_size = (size + 3) & !3;
        
        // Check if we have enough space
        if self.heap_pointer + aligned_size >= self.stack_base {
            return Err(VMError::OutOfMemory);
        }
        
        let address = self.heap_pointer;
        self.heap_pointer += aligned_size;
        
        // Track the allocation
        self.allocated_blocks.insert(address, aligned_size);
        
        Ok(address)
    }

    /// Free memory on the heap
    pub fn free(&mut self, address: u32) -> VMResult<()> {
        if let Some(size) = self.allocated_blocks.remove(&address) {
            // Zero out the freed memory for security
            for i in 0..size {
                if let Ok(()) = self.write_byte(address + i, 0) {
                    // Continue zeroing
                }
            }
            Ok(())
        } else {
            Err(VMError::FreeFailed(address))
        }
    }

    /// Check if an address is valid and allocated
    pub fn is_valid_address(&self, address: u32) -> bool {
        if address >= self.memory_size {
            return false;
        }
        
        // Check if it's in code section
        if address < self.heap_base {
            return true;
        }
        
        // Check if it's in an allocated heap block
        for (&block_addr, &block_size) in &self.allocated_blocks {
            if address >= block_addr && address < block_addr + block_size {
                return true;
            }
        }
        
        // Check if it's in stack space
        address >= self.stack_pointer && address < self.stack_base
    }

    /// Push a value onto the stack
    pub fn stack_push(&mut self, value: u32) -> VMResult<()> {
        if self.stack_pointer < self.heap_pointer + 4 {
            return Err(VMError::StackOverflow);
        }
        
        self.stack_pointer -= 4;
        self.write_word(self.stack_pointer, value)
    }

    /// Pop a value from the stack
    pub fn stack_pop(&mut self) -> VMResult<u32> {
        if self.stack_pointer >= self.stack_base {
            return Err(VMError::StackUnderflow);
        }
        
        let value = self.read_word(self.stack_pointer)?;
        self.stack_pointer += 4;
        Ok(value)
    }

    /// Get current stack pointer
    pub fn get_stack_pointer(&self) -> u32 {
        self.stack_pointer
    }

    /// Set stack pointer (use with caution)
    pub fn set_stack_pointer(&mut self, sp: u32) -> VMResult<()> {
        if sp > self.stack_base || sp < self.heap_pointer {
            return Err(VMError::InvalidMemoryAddress(sp));
        }
        self.stack_pointer = sp;
        Ok(())
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_memory: self.memory_size,
            heap_used: self.heap_pointer - self.heap_base,
            stack_used: self.stack_base - self.stack_pointer,
            allocated_blocks: self.allocated_blocks.len(),
            heap_fragmentation: self.calculate_fragmentation(),
        }
    }

    /// Calculate heap fragmentation (simple metric)
    fn calculate_fragmentation(&self) -> f32 {
        if self.allocated_blocks.is_empty() {
            return 0.0;
        }
        
        let total_allocated: u32 = self.allocated_blocks.values().sum();
        let heap_used = self.heap_pointer - self.heap_base;
        
        if heap_used == 0 {
            0.0
        } else {
            1.0 - (total_allocated as f32 / heap_used as f32)
        }
    }

    /// Reset memory state
    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.stack_pointer = self.stack_base;
        self.heap_pointer = self.heap_base;
        self.allocated_blocks.clear();
    }

    /// Dump memory contents for debugging
    pub fn dump_range(&self, start: u32, length: u32) -> String {
        let mut output = String::new();
        output.push_str(&format!("Memory dump from 0x{:08X} to 0x{:08X}:\n", 
                                start, start + length));
        
        for i in (0..length).step_by(16) {
            let addr = start + i;
            if addr >= self.memory_size {
                break;
            }
            
            output.push_str(&format!("{:08X}: ", addr));
            
            // Hex bytes
            for j in 0..16 {
                if addr + j < self.memory_size && i + j < length {
                    output.push_str(&format!("{:02X} ", self.memory[(addr + j) as usize]));
                } else {
                    output.push_str("   ");
                }
            }
            
            output.push_str(" |");
            
            // ASCII representation
            for j in 0..16 {
                if addr + j < self.memory_size && i + j < length {
                    let byte = self.memory[(addr + j) as usize];
                    if byte >= 32 && byte <= 126 {
                        output.push(byte as char);
                    } else {
                        output.push('.');
                    }
                } else {
                    output.push(' ');
                }
            }
            
            output.push_str("|\n");
        }
        
        output
    }
}

/// Memory statistics structure
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: u32,
    pub heap_used: u32,
    pub stack_used: u32,
    pub allocated_blocks: usize,
    pub heap_fragmentation: f32,
}

impl std::fmt::Display for MemoryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Memory Statistics:")?;
        writeln!(f, "  Total Memory: {} bytes ({:.1} MB)", 
                self.total_memory, self.total_memory as f32 / 1024.0 / 1024.0)?;
        writeln!(f, "  Heap Used: {} bytes ({:.1} KB)", 
                self.heap_used, self.heap_used as f32 / 1024.0)?;
        writeln!(f, "  Stack Used: {} bytes ({:.1} KB)", 
                self.stack_used, self.stack_used as f32 / 1024.0)?;
        writeln!(f, "  Allocated Blocks: {}", self.allocated_blocks)?;
        writeln!(f, "  Heap Fragmentation: {:.1}%", self.heap_fragmentation * 100.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new(1024 * 1024); // 1MB
        assert_eq!(memory.memory_size, 1024 * 1024);
        assert_eq!(memory.heap_base, 0x10000);
    }

    #[test]
    fn test_byte_operations() {
        let mut memory = Memory::new(1024);
        
        // Test write and read
        assert!(memory.write_byte(100, 42).is_ok());
        assert_eq!(memory.read_byte(100).unwrap(), 42);
        
        // Test invalid address
        assert!(memory.write_byte(2000, 1).is_err());
        assert!(memory.read_byte(2000).is_err());
    }

    #[test]
    fn test_word_operations() {
        let mut memory = Memory::new(1024);
        let test_value = 0x12345678;
        
        assert!(memory.write_word(100, test_value).is_ok());
        assert_eq!(memory.read_word(100).unwrap(), test_value);
        
        // Test boundary
        assert!(memory.write_word(1021, test_value).is_err());
    }

    #[test]
    fn test_stack_operations() {
        let mut memory = Memory::new(1024 * 1024);
        
        // Test push and pop
        assert!(memory.stack_push(42).is_ok());
        assert!(memory.stack_push(100).is_ok());
        
        assert_eq!(memory.stack_pop().unwrap(), 100);
        assert_eq!(memory.stack_pop().unwrap(), 42);
    }

    #[test]
    fn test_heap_allocation() {
        let mut memory = Memory::new(1024 * 1024);
        
        // Test allocation
        let addr1 = memory.allocate(100).unwrap();
        let addr2 = memory.allocate(200).unwrap();
        
        assert_ne!(addr1, addr2);
        assert!(memory.is_valid_address(addr1));
        assert!(memory.is_valid_address(addr2));
        
        // Test free
        assert!(memory.free(addr1).is_ok());
        assert!(memory.free(addr2).is_ok());
        
        // Test double free
        assert!(memory.free(addr1).is_err());
    }

    #[test]
    fn test_program_loading() {
        let mut memory = Memory::new(1024 * 1024);
        let program = vec![0x12345678, 0xABCDEF00, 0x11111111];
        
        assert!(memory.load_program(&program).is_ok());
        
        // Verify program was loaded correctly
        assert_eq!(memory.read_word(0).unwrap(), 0x12345678);
        assert_eq!(memory.read_word(4).unwrap(), 0xABCDEF00);
        assert_eq!(memory.read_word(8).unwrap(), 0x11111111);
    }
}