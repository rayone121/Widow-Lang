use crate::vm::error::{VMError, VMResult};

/// Register file containing 32 general-purpose registers
#[derive(Debug, Clone)]
pub struct RegisterFile {
    registers: [i32; 32],
}

impl RegisterFile {
    /// Create a new register file with all registers initialized to 0
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
        }
    }

    /// Read value from a register
    pub fn read(&self, reg: u8) -> VMResult<i32> {
        if reg >= 32 {
            return Err(VMError::InvalidRegister(reg));
        }
        Ok(self.registers[reg as usize])
    }

    /// Write value to a register
    pub fn write(&mut self, reg: u8, value: i32) -> VMResult<()> {
        if reg >= 32 {
            return Err(VMError::InvalidRegister(reg));
        }
        
        // Register 0 is typically read-only zero register in many architectures
        // Uncomment the following lines if you want R0 to always be zero:
        // if reg == 0 {
        //     return Ok(()); // Ignore writes to R0
        // }
        
        self.registers[reg as usize] = value;
        Ok(())
    }

    /// Get a reference to all registers (for debugging/inspection)
    pub fn get_all(&self) -> &[i32; 32] {
        &self.registers
    }

    /// Reset all registers to zero
    pub fn reset(&mut self) {
        self.registers = [0; 32];
    }

    /// Dump register state for debugging
    pub fn dump(&self) -> String {
        let mut output = String::new();
        output.push_str("Register File State:\n");
        
        for i in 0..32 {
            if i % 4 == 0 {
                output.push_str(&format!("R{:02}-R{:02}: ", i, (i + 3).min(31)));
            }
            
            output.push_str(&format!("R{:02}={:08X} ", i, self.registers[i] as u32));
            
            if (i + 1) % 4 == 0 || i == 31 {
                output.push('\n');
            }
        }
        
        output
    }

    /// Set register values from a slice (useful for testing/initialization)
    pub fn set_from_slice(&mut self, values: &[i32]) -> VMResult<()> {
        if values.len() > 32 {
            return Err(VMError::InvalidRegister(values.len() as u8));
        }
        
        for (i, &value) in values.iter().enumerate() {
            self.registers[i] = value;
        }
        
        Ok(())
    }

    /// Check if two register files are equal (useful for testing)
    pub fn equals(&self, other: &RegisterFile) -> bool {
        self.registers == other.registers
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_creation() {
        let regs = RegisterFile::new();
        for i in 0..32 {
            assert_eq!(regs.read(i).unwrap(), 0);
        }
    }

    #[test]
    fn test_register_read_write() {
        let mut regs = RegisterFile::new();
        
        // Test writing and reading
        assert!(regs.write(5, 42).is_ok());
        assert_eq!(regs.read(5).unwrap(), 42);
        
        // Test negative values
        assert!(regs.write(10, -123).is_ok());
        assert_eq!(regs.read(10).unwrap(), -123);
    }

    #[test]
    fn test_invalid_register() {
        let mut regs = RegisterFile::new();
        
        // Test invalid register read
        assert!(matches!(regs.read(32), Err(VMError::InvalidRegister(32))));
        assert!(matches!(regs.read(255), Err(VMError::InvalidRegister(255))));
        
        // Test invalid register write
        assert!(matches!(regs.write(32, 100), Err(VMError::InvalidRegister(32))));
        assert!(matches!(regs.write(50, 200), Err(VMError::InvalidRegister(50))));
    }

    #[test]
    fn test_register_reset() {
        let mut regs = RegisterFile::new();
        
        // Set some values
        regs.write(1, 100).unwrap();
        regs.write(5, 200).unwrap();
        regs.write(31, 300).unwrap();
        
        // Reset and verify all are zero
        regs.reset();
        for i in 0..32 {
            assert_eq!(regs.read(i).unwrap(), 0);
        }
    }

    #[test]
    fn test_set_from_slice() {
        let mut regs = RegisterFile::new();
        let values = [1, 2, 3, 4, 5];
        
        regs.set_from_slice(&values).unwrap();
        
        for (i, &expected) in values.iter().enumerate() {
            assert_eq!(regs.read(i as u8).unwrap(), expected);
        }
        
        // Remaining registers should still be 0
        for i in values.len()..32 {
            assert_eq!(regs.read(i as u8).unwrap(), 0);
        }
    }

    #[test]
    fn test_register_dump() {
        let mut regs = RegisterFile::new();
        regs.write(0, 0x12345678_u32 as i32).unwrap();
        regs.write(1, 0xABCDEF00_u32 as i32).unwrap();
        
        let dump = regs.dump();
        assert!(dump.contains("Register File State:"));
        assert!(dump.contains("12345678"));
        assert!(dump.contains("ABCDEF00"));
    }

    #[test]
    fn test_register_equality() {
        let mut regs1 = RegisterFile::new();
        let mut regs2 = RegisterFile::new();
        
        assert!(regs1.equals(&regs2));
        
        regs1.write(5, 42).unwrap();
        assert!(!regs1.equals(&regs2));
        
        regs2.write(5, 42).unwrap();
        assert!(regs1.equals(&regs2));
    }
}