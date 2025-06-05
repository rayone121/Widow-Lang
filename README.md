# 🕷️ Widow Programming Language

<div align="center">

![Widow Logo](https://img.shields.io/badge/Widow-Core%20VM%20%26%20Bytecode%20Implemented%20%7C%20Language%20Frontend%20In%20Development-8B5CF6?style=for-the-badge&logo=rust&logoColor=white)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge)](LICENSE)
[![VM Architecture](https://img.shields.io/badge/VM-Register%20Based-green?style=for-the-badge)](docs/architecture.md)

**A modern, bytecode-compiled programming language with garbage collection and expressive syntax. Currently under active development, with a functional core VM and bytecode execution layer. The language frontend (parser, compiler for `.wd` files) is the current focus.**

*Building for performance, designing for developers* 🚀

</div>

---

## ✨ Core Features & Goals

| Feature | Status | Description | Example (Target Syntax) |
|---------|--------|-------------|-------------------------|
| ⚡ **Register VM** | ✅ Implemented | Fast bytecode execution | Optimized for performance |
| 🔄 **Garbage Collection** | ✅ Implemented | Automatic memory management (tricolor mark-and-sweep, generational support) | No manual memory handling |
| 🔤 **High-Level Syntax** | 🚧 In Progress | Expressive and modern language features | `let name = "Widow"` |
| 🎯 **Type Safety** | 🎯 Goal | Optional static typing with inference | `x:i32 = 42` or `x = 42` |
| 📦 **Bytecode** | ✅ Implemented | Well-defined instruction set for the VM | See VM Demos |
| 🧩 **Language Frontend** | 🚧 In Progress | Lexer, Parser, AST, and Compiler for `.wd` source files | `.wd` files → Bytecode |
| 🧵 **Async/Await** | 🎯 Goal | Built-in concurrency support | `result = await fetch_data(url)` |
| 🔍 **Pattern Matching** | 🎯 Goal | Powerful destructuring | `match point { (x, y) => ... }` |

---

## 🚀 Quick Start

### Building from Source

Widow is currently in active development. To use it, you'll need to build from source:

```bash
git clone https://github.com/rayone121/Widow-Lang # Replace with your actual repo URL
cd Widow-Lang
cargo build --release 
# The main binary will be in target/release/widow
# (Note: direct .wd file execution is not yet supported)
```

### Current Capabilities: Direct Bytecode Execution

While the compiler for `.wd` source files is under development, the Virtual Machine can execute bytecode directly. You can construct bytecode using the `InstructionBuilder` and run it on the VM.

Here's a conceptual example in Rust (similar to internal demos):

```rust
// This is a Rust example showing how to interact with the VM
// import widow_lang::{VM, InstructionBuilder, encode};
// import widow_lang::compiler::instruction_builder::registers::*;

// fn run_simple_bytecode_demo() {
//     let mut vm = VM::new_default();
//
//     // Program: R0 = (10 + 5) * 3
//     let program_bytecode = vec![
//         encode(InstructionBuilder::load_immediate(r1(), 10)), // R1 = 10
//         encode(InstructionBuilder::load_immediate(r2(), 5)),  // R2 = 5
//         encode(InstructionBuilder::add(r3(), r1(), r2())),    // R3 = R1 + R2 = 15
//         encode(InstructionBuilder::load_immediate(r4(), 3)),  // R4 = 3
//         encode(InstructionBuilder::mul(r0(), r3(), r4())),    // R0 = R3 * R4 = 45
//         encode(InstructionBuilder::print(r0())),              // Print R0
//         encode(InstructionBuilder::halt()),
//     ];
//
//     vm.load_program(&program_bytecode).expect("Failed to load bytecode");
//     vm.run().expect("VM execution failed");
//     // Expected output: 45
// }
```
*(The above Rust snippet demonstrates how the VM can be used programmatically. The `main.rs` in the project contains runnable demos like this.)*

---

## 🎨 Language Highlights (Target Syntax)

**Note:** The following examples showcase the *target syntax* for Widow. The compiler and tooling to directly parse and execute these `.wd` files are currently under active development. The core VM and bytecode execution layer are functional.

### 🔤 Modern String Interpolation
```widow
name = "Alice"
age = 30
message = `Hello, ${name}! You are ${age} years old.`

// Multi-line with formatting
report = f`
    Name: ${name:>15}
    Age:  ${age:>15}
    Status: ${"Active":>12}
`
```

### 🎯 Smart Pattern Matching
```widow
result = match user_input {
    "quit", "exit", "q" => "Goodbye! 👋"
    n if is_number(n) && n > 100 => "Big number! 📈"
    email if email.contains("@") => `Email: ${email} 📧`
    _ => "I don't understand 🤔"
}
```

### ⚡ Async Programming Made Easy
```widow
async func fetch_user_data(id) {
    profile = await http.get(`/users/${id}`)
    posts = await http.get(`/users/${id}/posts`)
    
    ret {
        profile: profile,
        posts: posts
    }
}

// Usage
user_data = await fetch_user_data(123)
```

### 🔧 Flexible Error Handling
(Showcasing one or two planned styles, e.g., Result Types)
```widow
func safe_divide(a, b) -> Result<f64, String> {
    if b == 0 {
        ret Err("Division by zero! ⚠️")
    }
    ret Ok(a / b)
}

result = safe_divide(10, 2)
    .map(|x| x * 2)
    .unwrap_or(0)
```

---

## 🏗️ Advanced Features (Target Syntax)

**Note:** The features described below represent the design goals for Widow. Implementation is ongoing.

### 🎭 Traits and Generics
```widow
trait Display {
    func to_string(self) -> String
}

struct Point<T> {
    x: T
    y: T
}

impl<T> Display for Point<T> where T: Display {
    func to_string(self) -> String {
        ret `Point(${self.x}, ${self.y})`
    }
}
```

### 🔄 Powerful Destructuring
```widow
// Object destructuring with defaults
{name, age, city = "Unknown"} = user

// Array destructuring with rest
[first, second, ...rest] = items
```

### 🌊 Functional Programming
```widow
// Pipe operations for clean data flow
processed_data = raw_input
    |> trim
    |> split(",")
    |> map(parse_int)
    |> filter(|x| x > 0)
    |> reduce(|acc, x| acc + x, 0)
```

---

## 🏛️ Architecture

### 🔧 Register-Based Virtual Machine
- **Performance**: Aims for faster execution compared to stack-based VMs.
- **Optimization**: Designed to be suitable for complex optimizations.
- **Memory**: Efficient register allocation and usage.

### 🗑️ Garbage Collection
- **Automatic**: Tricolor mark-and-sweep algorithm.
- **Generational**: Support for generational collection to optimize for object lifetimes.
- **Concurrent Features**: Planned for future enhancements to minimize pause times.

### 📦 Bytecode Compilation Flow
```
Source Code (.wd) → Lexer → Parser → AST → Compiler → Bytecode → VM Execution
```
*(The VM, Bytecode definition, and direct execution are implemented. The Lexer, Parser, AST, and the Compiler (AST to Bytecode) for `.wd` files are under development.)*

---

## 📚 Documentation Structure (Planned)

| Section | Description | Link |
|---------|-------------|------|
| 📖 **Language Guide** | Complete syntax and features | Language Guide (Planned) |
| 🛠️ **API Reference** | Standard library documentation | API Reference (Planned) |
| 🏗️ **Architecture** | VM internals and design | [Architecture Details](docs/architecture.md) (To be created) |
| 🎯 **Examples** | Code samples in Widow | Examples (Planned) |
| 🤝 **Contributing** | How to contribute | [Contributing](CONTRIBUTING.md) |

---

## 🛣️ Roadmap

### ✅ Completed
- Core Register-Based Virtual Machine (VM)
- Bytecode Definition, Encoder, Decoder, and Interpreter
- Generational Garbage Collector (Tricolor Mark-and-Sweep)

### 🚧 Current Focus (Building the Language Frontend)
- [ ] **Lexer**: Tokenizing `.wd` source code.
- [ ] **Parser**: Generating Abstract Syntax Trees (AST) from tokens.
- [ ] **AST Design**: Defining the structure of the language constructs.

### 🎯 Next Steps
- [ ] **Compiler (AST to Bytecode)**: Translating the AST into executable VM bytecode.
- [ ] **Basic Standard Library**: Core functions for I/O, collections, etc.
- [ ] **Command-Line Interface (CLI)**: Basic tooling to run `.wd` files (once compiler is ready).

### 🔮 Future Goals
- Richer Type System and Type Inference
- Expanded Language Features (Concurrency, Advanced Error Handling, Macros)
- Comprehensive Standard Library
- Performance Optimizations (e.g., JIT considerations)
- Developer Tooling (Debugger, Package Manager, LSP)

---

## 📄 License

Widow is released under the [GNU General Public License v3.0](LICENSE). Feel free to use and contribute!

---

## 🙏 Acknowledgments

Special thanks to:
- 🦀 **Rust Community** for the amazing ecosystem.
- 🧠 **Language Design Inspiration** from Rust, Go, Python and JavaScript.
- 👥 **Early Contributors** and those who provide feedback.
- 💖 **You** for checking out Widow!

---

<div align="center">

**Built with ❤️ and ☕ by rayone121**

[⭐ Star us on GitHub](https://github.com/rayone121/Widow-Lang) 

</div>