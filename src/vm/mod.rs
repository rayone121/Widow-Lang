pub mod vm;
pub mod memory;
pub mod error;
pub mod registers;
pub mod gc;

pub use vm::VM;
pub use error::VMError;
pub use gc::{GarbageCollector, GCConfig, GCStats};