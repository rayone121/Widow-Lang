use crate::compiler::{inst_type::InstructionType, instruction::decode, opcode::Opcode};

pub struct VM {
    registers: [u32; 16],
    memory: Vec<u32>,
    pc: usize,
    running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 16], // Initialize all registers to 0
            memory: Vec::new(), // Empty memory initially
            pc: 0,              // Start at instruction 0
            running: false,     // VM not running yet
        }
    }

    pub fn load_program(&mut self, program: Vec<u32>) -> Result<(), String> {
        self.memory = program;
        self.pc = 0;
        self.running = false;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.running = true;
        while self.running {
            if self.pc >= self.memory.len() {
                return Err("Program counter out of bounds".to_string());
            }

            let instruction_bits = self.memory[self.pc];

            let instruction = decode(instruction_bits)?;

            self.execute_instruction(instruction)?;

            self.pc += 1;
        }
        Ok(())
    }

    pub fn set_register(&mut self, reg: usize, value: u32) -> Result<(), String> {
        if reg >= 16 {
            return Err(format!("Register {} out of range (0-15)", reg));
        }
        self.registers[reg] = value;
        Ok(())
    }

    pub fn get_register(&self, reg: usize) -> Result<u32, String> {
        if reg >= 16 {
            return Err(format!("Register {} out of range (0-15)", reg));
        }
        Ok(self.registers[reg])
    }

    fn execute_instruction(&mut self, instruction: InstructionType) -> Result<(), String> {
        match instruction {
            InstructionType::RType { opcode, rs, rt, rd, shamt, funct } => {
                match opcode {
                    Opcode::MOV => {
                        // MOV rd, rs - copy rs to rd
                        let src_value = self.registers[rs.get_value() as usize];
                        self.registers[rd.get_value() as usize] = src_value;
                        // rt, shamt, funct are unused in MOV but kept for R-Type format consistency
                        let _ = (rt, shamt, funct); // Suppress unused variable warnings
                    }
                    _ => return Err(format!("Unimplemented R-Type: {:?}", opcode)),
                }
            }
            InstructionType::NType { opcode } => {
                match opcode {
                    Opcode::NOP => { /* Do nothing */ }
                    Opcode::HALT => {
                        self.running = false;
                    }
                    _ => return Err(format!("Unimplemented N-Type: {:?}", opcode)),
                }
            }
            _ => return Err("Unimplemented instruction type".to_string()),
        }
        Ok(())
    }
}
