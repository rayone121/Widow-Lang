pub mod compiler;
pub mod vm;

pub use vm::{VM, VMError};
pub use compiler::instruction_builder::InstructionBuilder;
pub use compiler::encode::encode;
pub use compiler::decode::decode;