use widow_lang::compiler::{
    inst_type::InstructionType,
    instruction::encode,
    opcode::Opcode,
    types::{FunctionCode, Register, ShiftAmount},
    vm::VM,
};

fn main() -> Result<(), String> {
    let mut vm = VM::new();

    // Create MOV R2, R1 instruction (copy R1 to R2)
    let mov_instr = InstructionType::RType {
        opcode: Opcode::MOV,
        rs: Register::new(1)?, // source register R1
        rt: Register::new(0)?, // unused
        rd: Register::new(2)?, // destination register R2
        shamt: ShiftAmount::new(0)?,
        funct: FunctionCode::new(0)?,
    };

    let halt = InstructionType::NType {
        opcode: Opcode::HALT,
    };

    let program = vec![encode(mov_instr), encode(halt)];

    vm.load_program(program)?;

    // Set R1 to 42 for testing (we'll need a method to access registers)
    vm.set_register(1, 42)?;

    println!(
        "Before MOV: R1={}, R2={}",
        vm.get_register(1)?,
        vm.get_register(2)?
    );

    vm.run()?;

    println!(
        "After MOV: R1={}, R2={}",
        vm.get_register(1)?,
        vm.get_register(2)?
    );
    println!("VM executed successfully!");
    Ok(())
}
